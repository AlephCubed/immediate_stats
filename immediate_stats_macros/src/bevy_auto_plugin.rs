use darling::ast::NestedMeta;
use darling::{Error, FromMeta};
use proc_macro2::{Ident, TokenStream};
use quote::{ToTokens, format_ident, quote};
use syn::{DeriveInput, Expr, Meta, Path};

/// Returns code that will register stat resetting system(s) with Bevy Auto Plugin.
pub fn register_systems(input: &DeriveInput) -> darling::Result<TokenStream> {
    let struct_name = &input.ident;

    let mut auto_plugin_attributes = AutoPluginAttributes::new(struct_name);

    for attr in &input.attrs {
        if attr.path().is_ident("auto_component") {
            let plugin = PluginPath::from_meta(&attr.meta)?;
            auto_plugin_attributes.component_plugin = Some(plugin);
        } else if attr.path().is_ident("auto_resource")
            || attr.path().is_ident("auto_init_resource")
            || attr.path().is_ident("auto_insert_resource")
        {
            let plugin = PluginPath::from_meta(&attr.meta)?;
            auto_plugin_attributes.resource_plugin = Some(plugin);
        }
    }

    Ok(auto_plugin_attributes.into_token_stream())
}

pub struct AutoPluginAttributes<'a> {
    ident: &'a Ident,
    component_plugin: Option<PluginPath>,
    resource_plugin: Option<PluginPath>,
}

impl<'a> AutoPluginAttributes<'a> {
    pub fn new(ident: &'a Ident) -> Self {
        Self {
            ident,
            component_plugin: None,
            resource_plugin: None,
        }
    }
}

impl<'a> ToTokens for AutoPluginAttributes<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if let Some(plugin_path) = &self.component_plugin {
            let ident = &self.ident;
            let plugin = &plugin_path.0;
            let system_ident = format_ident!("__reset_{ident}_component_modifiers");

            tokens.extend(quote! {
                #[bevy_auto_plugin::prelude::auto_system(
                    plugin = #plugin,
                    schedule = immediate_stats::__PreUpdate,
                    config(
                        in_set = immediate_stats::StatSystems::Reset,
                    )
                )]
                fn #system_ident(
                    mut query: Query<&mut #ident, Without<immediate_stats::PauseStatReset>>,
                ) {
                    for mut stat in &mut query {
                        stat.reset_modifiers();
                    }
                }
            });
        }

        if let Some(plugin_path) = &self.resource_plugin {
            let ident = &self.ident;
            let plugin = &plugin_path.0;
            let system_ident = format_ident!("__reset_{ident}_resource_modifiers");

            tokens.extend(quote! {
                #[bevy_auto_plugin::prelude::auto_system(
                    plugin = #plugin,
                    schedule = immediate_stats::__PreUpdate,
                    config(
                        in_set = immediate_stats::StatSystems::Reset,
                    )
                )]
                fn #system_ident(res: Option<ResMut<#ident>>) {
                    if let Some(mut res) = res {
                        res.reset_modifiers();
                    }
                }
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
                        _ => Err(Error::custom("Expected a path to an auto plugin")),
                    },
                },
                NestedMeta::Lit(_) => Err(Error::custom("Expected `plugin` attribute")),
            };
        }

        Err(Error::custom("Expected `plugin` attribute"))
    }
}
