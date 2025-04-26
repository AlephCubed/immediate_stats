#[cfg(feature = "bevy")]
mod bevy;

use proc_macro_error::{
    emit_call_site_error, emit_call_site_warning, emit_warning, proc_macro_error,
};
use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::{Data, DataEnum, DataStruct, DeriveInput, Field, Ident, Index};

#[proc_macro_derive(StatContainer, attributes(stat, stat_ignore, add_component))]
#[proc_macro_error]
pub fn stat_container_derive(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let tree: DeriveInput = syn::parse(item).expect("TokenStream must be valid.");

    let struct_name = &tree.ident;

    let method = match tree.data.clone() {
        Data::Struct(s) => stat_container_struct(s),
        Data::Enum(e) => stat_container_enum(e),
        Data::Union(_) => {
            emit_call_site_error!("This trait cannot be derived for unions.");
            return proc_macro::TokenStream::new();
        }
    };

    let result = quote! {
        impl StatContainer for #struct_name {
            #method
        }
    };

    #[cfg(feature = "bevy_butler")]
    {
        let systems = bevy::butler::register_butler_systems(&tree);
        quote! { #result #systems }.into()
    }

    #[cfg(not(feature = "bevy_butler"))]
    result.into()
}

fn stat_container_struct(s: DataStruct) -> TokenStream {
    match get_members_from_fields(s.fields) {
        MemberVec::Named(names) => {
            quote! {
                fn reset_modifiers(&mut self) {
                    #(self.#names.reset_modifiers();)*
                }
            }
        }
        MemberVec::Unnamed(nums) => {
            quote! {
                fn reset_modifiers(&mut self) {
                    #(self.#nums.reset_modifiers();)*
                }
            }
        }
        MemberVec::None => {
            emit_call_site_warning!(
                "Unused derive. Consider adding `#[stat]` to a field that implements `StatContainer`."
            );
            TokenStream::new()
        }
    }
}

fn stat_container_enum(e: DataEnum) -> TokenStream {
    let mut cases = Vec::new();

    for variant in e.variants {
        let ident = variant.ident;

        match get_members_from_fields(variant.fields.clone()) {
            MemberVec::Named(names) => {
                cases.push(quote! {
                    Self::#ident { #(#names,)* .. } => {
                        #(#names.reset_modifiers();)*
                    },
                });
            }
            MemberVec::Unnamed(nums) => {
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

                if variables.is_empty() {
                    continue;
                }

                let used_vars = variables.iter().filter(|x| x.to_string() != "_");

                cases.push(quote! {
                    Self::#ident(#(#variables,)*) => {
                        #(#used_vars.reset_modifiers();)*
                    },
                });
            }
            MemberVec::None => {}
        }
    }

    if cases.is_empty() {
        emit_call_site_warning!(
            "Unused derive. Consider adding `#[stat]` to a field that implements `StatContainer`."
        );
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

/// The ways to identify fields.
enum MemberVec {
    Named(Vec<Ident>),
    Unnamed(Vec<Index>),
    None,
}

/// Returns a list of all fields that either have type `Stat` or are tagged with `#[stat]`.
/// If they are tagged with `#[stat_ignore]`, they are removed from the list.
fn get_members_from_fields<T: IntoIterator<Item = Field>>(fields: T) -> MemberVec {
    let mut names = Vec::new();
    let mut nums = Vec::new();

    for (index, field) in fields.into_iter().enumerate() {
        // Check if the field is a stat.
        let mut is_stat = false;

        // Check if type is `Stat`.
        if field.ty.to_token_stream().to_string().contains("Stat") {
            is_stat = true;
        }

        // Store the `#[stat]` ident. Used for warning when overridden by `#[stat_ignore]`.
        let mut explicit_stat: Option<Ident> = None;

        // Iterator over all ident attributes.
        let attr_ident_iter = field.attrs.iter().filter_map(|x| x.meta.path().get_ident());

        // Check for `#[stat]` attribute.
        if let Some(attr_ident) = attr_ident_iter.clone().find(|x| x.to_string() == "stat") {
            if is_stat {
                emit_warning!(
                    attr_ident,
                    "Unnecessary `stat` attribute. Fields of type `Stat` are automatically included."
                );
            }

            is_stat = true;
            explicit_stat = Some(attr_ident.clone());
        }

        // Check for `#[stat_ignore]` attribute.
        if attr_ident_iter
            .clone()
            .find(|x| x.to_string() == "stat_ignore")
            .is_some()
        {
            if let Some(explicit_ident) = &explicit_stat {
                emit_warning!(
                    explicit_ident,
                    "`stat` attribute is overruled by `stat_ignore` attribute."
                );
            }

            is_stat = false;
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

    assert!(names.is_empty() | nums.is_empty());

    if names.is_empty() {
        MemberVec::Unnamed(nums)
    } else if nums.is_empty() {
        MemberVec::Named(names)
    } else {
        MemberVec::None
    }
}
