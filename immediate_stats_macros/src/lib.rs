#[cfg(feature = "bevy_butler")]
mod bevy_butler;

use darling::{Error, FromField};
use proc_macro_error::{emit_call_site_error, emit_warning, proc_macro_error};
use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Field, Fields, Ident, Index, Variant, parse_macro_input};

#[proc_macro_derive(StatContainer, attributes(stat, stat_ignore, add_component))]
#[proc_macro_error]
pub fn stat_container_derive(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let tree: DeriveInput = parse_macro_input!(item as DeriveInput);
    let ident = &tree.ident;

    let method_contents = match tree.data.clone() {
        Data::Struct(s) => reset_struct(&s.fields),
        Data::Enum(e) => reset_variants(e.variants.iter()),
        Data::Union(_) => {
            emit_call_site_error!("This trait cannot be derived from unions.");
            return proc_macro::TokenStream::new();
        }
    };

    let method = quote! {
        impl StatContainer for #ident {
            fn reset_modifiers(&mut self) {
                #method_contents
            }
        }
    };

    #[cfg(feature = "bevy_butler")]
    {
        let systems = bevy_butler::register_systems(tree).unwrap_or_else(Error::write_errors);
        quote! { #method #systems }.into()
    }

    #[cfg(not(feature = "bevy_butler"))]
    method.into()
}

#[derive(Default)]
struct FieldOptions {
    ident: Option<Ident>,
    stat_type: bool,
    include: bool,
    exclude: bool,
}

impl FieldOptions {
    /// Returns true if the field is considered a stat.
    /// Emits a warning if both the `stat` and `stat_ignore` flags are present.
    pub fn is_stat(&self) -> bool {
        if self.include && self.exclude {
            emit_warning!(
                self.ident.span(),
                "`stat` attribute is overruled by `stat_ignore` attribute."
            );
        }

        (self.include || self.stat_type) && !self.exclude
    }
}

impl FromField for FieldOptions {
    fn from_field(field: &Field) -> darling::Result<Self> {
        let mut options = FieldOptions {
            ident: field.ident.clone(),
            ..Self::default()
        };

        options.stat_type = field.ty.to_token_stream().to_string().contains("Stat");

        for attribute in &field.attrs {
            if let Some(ident) = attribute.path().get_ident() {
                match ident.to_string().as_str() {
                    // Todo Warn about double tags.
                    "stat" => options.include = true,
                    "stat_ignore" => options.exclude = true,
                    _ => continue,
                }
            }
        }

        Ok(options)
    }
}

/// Returns the code that can be used to reset a struct's stat fields.
fn reset_struct(fields: &Fields) -> TokenStream {
    fields
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
    let options = FieldOptions::from_field(&field).unwrap();

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

/// Returns a match statement that can be used to reset an enum.
fn reset_variants<'a, T>(variants: T) -> TokenStream
where
    T: Iterator<Item = &'a Variant>,
{
    variants
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
    let names: Vec<Ident> = variant
        .fields
        .iter()
        .enumerate()
        .filter_map(|(index, field)| {
            let options = FieldOptions::from_field(field).unwrap();

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
