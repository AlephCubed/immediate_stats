use proc_macro_error::emit_call_site_warning;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Token};

pub(super) fn register_butler_systems(tree: &DeriveInput) -> TokenStream {
    let mut systems = Vec::new();

    for attr in &tree.attrs {
        let struct_name = &tree.ident;

        let path = attr.meta.path();

        let Some(ident) = path.get_ident() else {
            continue;
        };

        match ident.to_string().as_str() {
            "resource" => {
                let _ = attr.parse_nested_meta(|meta| {
                    let Some(var_name) = meta.path.segments.first() else {
                        return Ok(())
                    };
                    
                    if var_name.ident.to_string() != "plugin" {
                        return Ok(())
                    };
                    
                    let input = &meta.input;
                    
                    input.parse::<Token![=]>().expect("An equals sign.");
                    let input = input.parse::<proc_macro2::Ident>().expect("An identifier.");
                    
                    emit_call_site_warning!("{:?}", input);
                    
                    systems.push(quote! {
                        #[bevy_butler::system(generics = <#input>, plugin = #path, schedule = bevy_app::PreUpdate)]
                        use crate::bevy::reset_resource_modifiers;
                    });
                    
                    Ok(())
                });
            }
            _ => continue,
        }
    }

    quote! { #(#systems)* }
}
