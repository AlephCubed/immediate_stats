use crate::FieldOptions;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{DataEnum, Ident, Variant};

/// Returns a match statement that can be used to reset an enum.
pub fn reset_enum(body: &DataEnum) -> TokenStream {
    body.variants
        .iter()
        .map(|variant| {
            let cases = reset_variant(&variant);
            quote! {
                match self {
                    #cases
                    _ => {}
                }
            }
        })
        .flatten()
        .collect()
}

/// Returns a case that can be used to reset the variant.
/// If there are no stat fields, the result will be empty.
fn reset_variant(variant: &Variant) -> TokenStream {
    // List of all identifiers that need to be reset.
    let names: Vec<Ident> = variant
        .fields
        .iter()
        .enumerate()
        .filter_map(|(index, field)| {
            let options = FieldOptions::from_field(field);

            if options.is_stat() {
                return Some(match options.ident {
                    Some(ident) => ident,
                    None => get_ident_from_index(index),
                });
            }

            None
        })
        .collect();

    if names.is_empty() {
        return TokenStream::new();
    }

    let is_named = variant.fields.iter().next().unwrap().ident.is_some();
    let ident = &variant.ident;

    if is_named {
        quote! {
            Self::#ident { #(#names,)* .. } => {
                #(#names.reset_modifiers();)*
            },
        }
    } else {
        quote! {
            Self::#ident ( #(#names,)* .. ) => {
                #(#names.reset_modifiers();)*
            },
        }
    }
}

/// Generates an alphabetic identifier from an index.
fn get_ident_from_index(index: usize) -> Ident {
    Ident::new(
        format!("{}", ('a' as u8 + index as u8) as char).as_str(),
        Span::call_site(),
    )
}
