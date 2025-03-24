mod stat_attributes;

use crate::stat_attributes::{AttrType, Stat};
use proc_macro2::{Ident, Span};
use quote::quote;
use std::collections::HashMap;
use syn::{Data, DeriveInput};

#[proc_macro_derive(StatContainer, attributes(base, bonus, multiplier, sub_stat))]
pub fn stat_container_derive(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let tree: DeriveInput = syn::parse(item).expect("A valid TokenStream");

    let struct_name = &tree.ident;

    let mut stats: HashMap<String, Stat> = HashMap::new();
    let mut sub_stats = Vec::new();

    match tree.data {
        Data::Struct(s) => {
            for field in s.fields {
                // (health_base, damage_base, etc.)
                let field_ident = field
                    .ident
                    .expect("Field must have an identifier. Cannot be used on tuples.");

                for attr in field.attrs {
                    let path = attr.meta.path();

                    // (base, bonus, or multiplier)
                    let Ok(attr_ident) = path.require_ident() else {
                        continue;
                    };
                    let Ok(attr_type) = AttrType::try_from(attr_ident) else {
                        if attr_ident.to_string() == "sub_stat" {
                            sub_stats.push(field_ident.clone());
                        }
                        continue;
                    };

                    // (health, damage, etc.)
                    let mut stat_ident = String::new();

                    attr.parse_nested_meta(|meta| {
                        stat_ident = meta.path.get_ident().unwrap().to_string();
                        Ok(())
                    })
                    .unwrap();

                    let stat = match stats.get_mut(&stat_ident) {
                        Some(s) => s,
                        None => {
                            stats.insert(stat_ident.clone(), Stat::default());
                            stats.get_mut(&stat_ident).unwrap()
                        }
                    };

                    stat.set(attr_type, field_ident.clone());
                }
            }
        }
        Data::Enum(_) => todo!(),
        Data::Union(_) => unimplemented!(),
    }

    let bonuses = stats.values().filter_map(|x| x.bonus.clone());
    let multipliers = stats.values().filter_map(|x| x.multiplier.clone());

    let methods = stats.iter().map(|(name, stat)| {
        let name = Ident::new(name, Span::call_site());
        let calculation = stat.total_calculation();

        quote! {
            pub fn #name(&self) -> i32 {
                #calculation
            }
        }
    });

    quote! {
        impl StatContainer for #struct_name {
            fn reset_modifiers(&mut self) {
                #(self.#bonuses = 0;)*
                #(self.#multipliers = 1.0;)*
                #(self.#sub_stats.reset_modifiers();)*
            }
        }

        impl #struct_name {
            #(#methods)*
        }
    }
    .into()
}
