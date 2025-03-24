use proc_macro2::{Ident, TokenStream};
use quote::quote;

/// Stores the identifiers for the individual stat fields.
#[derive(Default)]
pub(super) struct Stat {
    pub base: Option<Ident>,
    pub bonus: Option<Ident>,
    pub multiplier: Option<Ident>,
}

impl Stat {
    pub(super) fn set(&mut self, attr_type: AttrType, ident: Ident) {
        match attr_type {
            AttrType::Base => self.base = Some(ident),
            AttrType::Bonus => self.bonus = Some(ident),
            AttrType::Multiplier => self.multiplier = Some(ident),
        }
    }

    /// Create the calculation for stat total.
    /// Results in `(self.base + self.bonus) * self.multiplier`
    /// with type conversions and missing values removed.
    /// # Panics
    /// When `self.base` is `None`.
    pub(super) fn total_calculation(&self) -> TokenStream {
        let result = self.base.clone().expect("All stats require a base!");
        let mut result = quote! { self.#result };

        if let Some(bonus) = &self.bonus {
            result = quote! {
                #result + self.#bonus
            }
        }

        if let Some(multiplier) = &self.multiplier {
            result = quote! {
                ((#result) as f32 * self.#multiplier) as i32
            }
        }

        result
    }
}

/// The types of stat fields.
pub(super) enum AttrType {
    Base,
    Bonus,
    Multiplier,
}

impl TryFrom<&String> for AttrType {
    type Error = ();

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "base" => Ok(Self::Base),
            "bonus" => Ok(Self::Bonus),
            "multiplier" => Ok(Self::Multiplier),
            _ => Err(()),
        }
    }
}

impl TryFrom<&Ident> for AttrType {
    type Error = ();

    fn try_from(value: &Ident) -> Result<Self, Self::Error> {
        Self::try_from(&value.to_string())
    }
}
