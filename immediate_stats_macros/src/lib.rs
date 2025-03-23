use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use std::collections::HashMap;
use syn::{Data, DeriveInput};

#[derive(Default)]
struct Stat {
    base: Option<Ident>,
    bonus: Option<Ident>,
    multiplier: Option<Ident>,
}

impl Stat {
    fn set(&mut self, attr_type: AttrType, ident: Ident) {
        match attr_type {
            AttrType::Base => self.base = Some(ident),
            AttrType::Bonus => self.bonus = Some(ident),
            AttrType::Multiplier => self.multiplier = Some(ident),
        }
    }
}

impl Stat {
    /// Create the calculation for stat total.
    /// Results in `(self.base + self.bonus) * self.multiplier`
    /// with type conversions and missing values removed.
    /// # Panics
    /// When `self.base` is `None`.
    fn total_calculation(&self) -> TokenStream {
        let result = self.base.clone().expect("All stats require a base!");
        let mut result = quote! { self.#result };

        if let Some(bonus) = &self.bonus {
            result = quote! {
                #result + self.#bonus
            }
        }

        if let Some(multiplier) = &self.multiplier {
            result = quote! {
                ((#result) as f32 * self.#multiplier) as i32
            }
        }

        result
    }
}

enum AttrType {
    Base,
    Bonus,
    Multiplier,
}

impl TryFrom<&String> for AttrType {
    type Error = ();

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "base" => Ok(Self::Base),
            "bonus" => Ok(Self::Bonus),
            "multiplier" => Ok(Self::Multiplier),
            _ => Err(()),
        }
    }
}

impl TryFrom<&Ident> for AttrType {
    type Error = ();

    fn try_from(value: &Ident) -> Result<Self, Self::Error> {
        Self::try_from(&value.to_string())
    }
}

#[proc_macro_derive(StatContainer, attributes(base, bonus, multiplier))]
pub fn stat_container_derive(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let tree: DeriveInput = syn::parse(item).expect("A valid TokenStream");

    let struct_name = &tree.ident;

    let mut stats: HashMap<String, Stat> = HashMap::new();

    match tree.data {
        Data::Struct(s) => {
            for field in s.fields {
                let ident = field.ident.unwrap();

                for attr in field.attrs {
                    let path = attr.meta.path();

                    let Ok(attr_ident) = path.require_ident() else {
                        continue;
                    };
                    let Ok(attr_type) = AttrType::try_from(attr_ident) else {
                        continue;
                    };

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

                    stat.set(attr_type, ident.clone());
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
            }
        }

        impl #struct_name {
            #(#methods)*
        }
    }
    .into()
}
