use darling::ast::NestedMeta;
use darling::{Error, FromMeta};
use proc_macro2::{Ident, TokenStream};
use quote::{ToTokens, format_ident, quote};
use syn::{DeriveInput, Expr, Meta, Path};

/// Returns code that will register stat resetting system(s) with Bevy Butler.
pub fn register_systems(input: DeriveInput) -> darling::Result<TokenStream> {
    let struct_name = &input.ident;

    let mut butler_attributes = ButlerAttributes::new(struct_name);

    for attr in input.attrs {
        if attr.path().is_ident("add_component") {
            let plugin = PluginPath::from_meta(&attr.meta)?;
            butler_attributes.component_plugin = Some(plugin);
        } else if attr.path().is_ident("insert_resource") {
            let plugin = PluginPath::from_meta(&attr.meta)?;
            butler_attributes.resource_plugin = Some(plugin);
        }
    }

    Ok(butler_attributes.into_token_stream())
}

pub struct ButlerAttributes<'a> {
    ident: &'a Ident,
    component_plugin: Option<PluginPath>,
    resource_plugin: Option<PluginPath>,
}

impl<'a> ButlerAttributes<'a> {
    pub fn new(ident: &'a Ident) -> Self {
        Self {
            ident,
            component_plugin: None,
            resource_plugin: None,
        }
    }
}

impl<'a> ToTokens for ButlerAttributes<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if let Some(plugin_path) = &self.component_plugin {
            let ident = &self.ident;
            let plugin = &plugin_path.0;
            let use_as = format_ident!("__{ident}_component");

            // Due to some strange import scoping issues, we cannot use the plugins.
            // Instead, we can just recreate the plugin's functionality.
            tokens.extend(quote! {
                #[bevy_butler::add_system(
                    generics = <#ident>,
                    plugin = #plugin,
                    schedule = immediate_stats::__PreUpdate,
                    in_set = immediate_stats::StatSystems::Reset,
                )]
                use immediate_stats::reset_component_modifiers as #use_as;
            });
        }

        if let Some(plugin_path) = &self.resource_plugin {
            let ident = &self.ident;
            let plugin = &plugin_path.0;
            let use_as = format_ident!("__{ident}_resource");

            // Due to some strange import scoping issues, we cannot use the plugins.
            // Instead, we can just recreate the plugin's functionality.
            tokens.extend(quote! {
                #[bevy_butler::add_system(
                    generics = <#ident>,
                    plugin = #plugin,
                    schedule = immediate_stats::__PreUpdate,
                    in_set = immediate_stats::StatSystems::Reset,
                )]
                use immediate_stats::reset_resource_modifiers as #use_as;
            });
        }
    }
}

/// Represents a `plugin(PATH)` or `plugin = PATH` attribute meta.
pub struct PluginPath(pub Path);

impl FromMeta for PluginPath {
    fn from_list(items: &[NestedMeta]) -> darling::Result<Self> {
        for item in items {
            return match item {
                NestedMeta::Meta(meta) => match meta {
                    Meta::Path(_) => Err(Error::custom("Expected a value for `plugin`")),
                    Meta::List(list) => {
                        if list.path.require_ident()? != "plugin" {
                            continue;
                        }

                        let mut path = None;

                        list.parse_nested_meta(|value_meta| {
                            path = Some(value_meta.path);
                            Ok(())
                        })?;

                        match path {
                            None => Err(Error::custom("Expected `plugin` attribute")),
                            Some(path) => Ok(PluginPath(path)),
                        }
                    }
                    Meta::NameValue(name_value) => match &name_value.value {
                        Expr::Path(p) => Ok(PluginPath(p.path.clone())),
                        _ => Err(Error::custom("Expected a path to a butler plugin")),
                    },
                },
                NestedMeta::Lit(_) => Err(Error::custom("Expected `plugin` attribute")),
            };
        }

        Err(Error::custom("Expected `plugin` attribute"))
    }
}
