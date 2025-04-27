use crate::FieldOptions;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DataStruct, Field, Index};

/// Returns the code that can be used to reset a struct's stat fields.
pub fn reset_struct(body: &DataStruct) -> TokenStream {
    body.fields
        .iter()
        .enumerate()
        .map(|(index, field)| reset_struct_field(field, index))
        .flatten()
        .collect()
}

/// Returns the method call that can be used to reset a stat field.
/// If the field is not a stat, the result will be empty.
/// The `index`  is used for tuple/unnamed fields.
fn reset_struct_field(field: &Field, index: usize) -> TokenStream {
    let options = FieldOptions::from_field(&field);

    if options.is_stat() {
        return match options.ident {
            Some(ident) => {
                quote! { self.#ident.reset_modifiers(); }
            }
            None => {
                let index = Index::from(index);
                quote! { self.#index.reset_modifiers(); }
            }
        };
    }

    TokenStream::new()
}
