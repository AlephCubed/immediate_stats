#[cfg(feature = "bevy_auto_plugin")]
mod bevy_auto_plugin;
mod derive_enum;
mod derive_struct;

use proc_macro_error::{emit_call_site_error, emit_warning, proc_macro_error};
use quote::{ToTokens, quote};
use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Field, Ident, parse_macro_input};

#[proc_macro_derive(StatContainer, attributes(stat, stat_ignore, add_component))]
#[proc_macro_error]
pub fn stat_container_derive(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let tree: DeriveInput = parse_macro_input!(item as DeriveInput);
    let ident = &tree.ident;

    let method_contents = match tree.data.clone() {
        Data::Struct(s) => derive_struct::reset_struct(&s),
        Data::Enum(e) => derive_enum::reset_enum(&e),
        Data::Union(_) => {
            emit_call_site_error!("This trait cannot be derived from unions.");
            return proc_macro::TokenStream::new();
        }
    };

    let trait_impl = quote! {
        impl StatContainer for #ident {
            fn reset_modifiers(&mut self) {
                #method_contents
            }
        }
    };

    #[cfg(feature = "bevy_auto_plugin")]
    {
        let systems =
            bevy_auto_plugin::register_systems(&tree).unwrap_or_else(darling::Error::write_errors);
        quote! { #trait_impl #systems }.into()
    }

    #[cfg(not(feature = "bevy_auto_plugin"))]
    trait_impl.into()
}

/// Represents the options that a field could have.
#[derive(Default)]
struct FieldOptions {
    ident: Option<Ident>,
    /// True if the field's type contains the word "Stat".
    stat_type: bool,
    /// True if the field has the `#[stat]` attribute.
    include: bool,
    /// True if the field has the `#[stat_ignore]` attribute.
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

    fn from_field(field: &Field) -> Self {
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

        options
    }
}
