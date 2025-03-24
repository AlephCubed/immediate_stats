use proc_macro_error::{
    emit_call_site_error, emit_call_site_warning, emit_warning, proc_macro_error,
};
use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::{Data, DataEnum, DataStruct, DeriveInput, Field, Ident, Index};

// Todo Return one or the other.
fn get_names_from_fields<T: IntoIterator<Item = Field>>(fields: T) -> (Vec<Ident>, Vec<Index>) {
    let mut names = Vec::new();
    let mut nums = Vec::new();

    for (index, field) in fields.into_iter().enumerate() {
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
                    emit_warning!(
                        ident,
                        "Unnecessary `stat` attribute. Fields of type `Stat` are automatically included."
                    );
                }

                is_stat = true;
            }
        }

        if !is_stat {
            continue;
        }

        // Add field to the list.
        if let Some(field_ident) = field.ident.clone() {
            // (health_base, damage_base, etc.)
            names.push(field_ident);
        } else {
            // Is tuple.
            nums.push(Index::from(index));
        }
    }

    (names, nums)
}

#[proc_macro_derive(StatContainer, attributes(stat))]
#[proc_macro_error]
pub fn stat_container_derive(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let tree: DeriveInput = syn::parse(item).expect("A valid TokenStream");

    let struct_name = &tree.ident;

    let method = match tree.data {
        Data::Struct(s) => stat_container_struct(s),
        Data::Enum(e) => stat_container_enum(e),
        Data::Union(_) => {
            emit_call_site_error!("This trait cannot be derived for unions");
            return proc_macro::TokenStream::new();
        }
    };

    quote! {
        impl StatContainer for #struct_name {
            #method
        }
    }
    .into()
}

fn stat_container_struct(s: DataStruct) -> TokenStream {
    let (names, nums) = get_names_from_fields(s.fields);

    if names.is_empty() && nums.is_empty() {
        emit_call_site_warning!(
            "Unused derive. Consider adding `#[stat]` to a field that implements `StatContainer`."
        );
    }

    quote! {
        fn reset_modifiers(&mut self) {
            #(self.#names.reset_modifiers();)*
            #(self.#nums.reset_modifiers();)*
        }
    }
}

fn stat_container_enum(e: DataEnum) -> TokenStream {
    let mut cases = Vec::new();

    for variant in e.variants {
        let ident = variant.ident;
        let (names, nums) = get_names_from_fields(variant.fields.clone());

        if !names.is_empty() {
            assert!(nums.is_empty());

            cases.push(quote! {
                Self::#ident { #(#names,)* .. } => {
                    #(#names.reset_modifiers();)*
                },
            });
        } else if !nums.is_empty() {
            assert!(names.is_empty());

            let mut variables = Vec::new();

            for index in 0..variant.fields.len() {
                if nums.contains(&Index::from(index)) {
                    variables.push(Ident::new(
                        format!("{}", ('a' as u8 + index as u8) as char).as_str(),
                        Span::call_site(),
                    ))
                } else {
                    variables.push(Ident::new("_", Span::call_site()));
                }
            }

            let used_vars = variables.iter().filter(|x| x.to_string() != "_");

            cases.push(quote! {
                Self::#ident(#(#variables,)*) => {
                    #(#used_vars.reset_modifiers();)*
                },
            });
        }
    }

    quote! {
        fn reset_modifiers(&mut self) {
            match self {
                #(#cases)*
                _ => {},
            }
        }
    }
}
