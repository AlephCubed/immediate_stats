use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Token};

pub(crate) fn register_butler_systems(tree: &DeriveInput) -> TokenStream {
    let mut systems = Vec::new();

    for attr in &tree.attrs {
        let struct_name = &tree.ident;

        let path = attr.meta.path();

        let Some(ident) = path.get_ident() else {
            continue;
        };

        let use_system = match ident.to_string().as_str() {
            "add_resource" => quote! { use immediate_stats::bevy::reset_resource_modifiers; },
            "add_component" => {
                quote! { use immediate_stats::bevy::reset_component_modifiers; }
            }
            _ => continue,
        };

        attr.parse_nested_meta(|meta| {
            let Some(var_name) = meta.path.segments.first() else {
                return Ok(())
            };

            if var_name.ident.to_string() != "plugin" {
                return Ok(())
            };

            let input = &meta.input;

            input.parse::<Token![=]>().expect("An equals sign.");
            let input = input.parse::<proc_macro2::Ident>().expect("An identifier.");

            systems.push(quote! {
                #[bevy_butler::add_system(generics = <#struct_name>, plugin = #input, schedule = immediate_stats::PreUpdate)]
                #use_system
            });

            Ok(())
        }).unwrap();
    }

    quote! { #(#systems)* }
}
