use quote::{ToTokens, quote};
use syn::{Data, DeriveInput, Index};

#[proc_macro_derive(StatContainer, attributes(sub_stat))]
pub fn stat_container_derive(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let tree: DeriveInput = syn::parse(item).expect("A valid TokenStream");

    let struct_name = &tree.ident;

    let mut stats = Vec::new();
    let mut nums = Vec::new();

    match tree.data {
        Data::Struct(s) => {
            for (index, field) in s.fields.iter().enumerate() {
                if field.ty.to_token_stream().to_string() == "Stat" {
                    if let Some(field_ident) = field.ident.clone() {
                        // (health_base, damage_base, etc.)
                        stats.push(field_ident);
                    } else {
                        // Is tuple.
                        nums.push(Index::from(index));
                    }
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
