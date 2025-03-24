use quote::{ToTokens, quote};
use syn::{Data, DeriveInput, Index};

#[proc_macro_derive(StatContainer, attributes(stat))]
pub fn stat_container_derive(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let tree: DeriveInput = syn::parse(item).expect("A valid TokenStream");

    let struct_name = &tree.ident;

    let mut stats = Vec::new();
    let mut nums = Vec::new();

    match tree.data {
        Data::Struct(s) => {
            for (index, field) in s.fields.iter().enumerate() {
                // Check if the field is a stat.
                let mut is_stat = false;

                if field.ty.to_token_stream().to_string() == "Stat" {
                    is_stat = true;
                }

                for attr in &field.attrs {
                    let path = attr.meta.path();
                    let Some(ident) = path.get_ident() else {
                        continue;
                    };

                    if ident.to_string() == "stat" {
                        if is_stat {
                            // Todo Warn about unneeded attribute.
                        } else {
                            is_stat = true;
                        }
                    }
                }

                if !is_stat {
                    continue;
                }

                // Add field to the list.
                if let Some(field_ident) = field.ident.clone() {
                    // (health_base, damage_base, etc.)
                    stats.push(field_ident);
                } else {
                    // Is tuple.
                    nums.push(Index::from(index));
                }
            }
        }
        Data::Enum(_) => todo!(),
        Data::Union(_) => unimplemented!(),
    }

    quote! {
        impl StatContainer for #struct_name {
            fn reset_modifiers(&mut self) {
                #(self.#stats.reset_modifiers();)*
                #(self.#nums.reset_modifiers();)*
            }
        }
    }
    .into()
}
