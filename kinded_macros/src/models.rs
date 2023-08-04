use proc_macro2::Ident;
use quote::format_ident;
use syn::{Generics, Path, Visibility};

#[derive(Debug)]
pub struct Meta {
    /// Visibility of enum.
    /// Kind implementation inherits this visibility automatically.
    pub vis: Visibility,

    pub ident: Ident,

    pub generics: Generics,

    pub variants: Vec<Variant>,

    /// Attributes specified with #[kinded(..)] above the enum definition.
    pub kinded_attrs: KindedAttributes,
}

impl Meta {
    /// Get the name for the kind type.
    pub fn kind_name(&self) -> Ident {
        if let Some(ref kind_name) = self.kinded_attrs.kind {
            kind_name.clone()
        } else {
            format_ident!("{}Kind", self.ident)
        }
    }

    /// Get the traits that need to be derived.
    pub fn derive_traits(&self) -> Vec<Path> {
        const DEFAULT_DERIVE_TRAITS: &[&str] = &["Debug", "Clone", "Copy", "PartialEq", "Eq"];

        let mut traits: Vec<Path> = DEFAULT_DERIVE_TRAITS
            .iter()
            .map(|trait_name| Path::from(format_ident!("{trait_name}")))
            .collect();

        // Add the extra specified traits, if they're different from the default ones
        if let Some(ref extra_traits) = self.kinded_attrs.derive {
            for extra_trait in extra_traits {
                if !traits.contains(extra_trait) {
                    traits.push(extra_trait.clone());
                }
            }
        }

        traits
    }
}

#[derive(Debug)]
pub struct Variant {
    pub ident: Ident,
    pub fields_type: FieldsType,
}

/// This mimics syn::Fields, but without payload.
#[derive(Debug)]
pub enum FieldsType {
    /// Example: `Admin { id: i32 }`
    Named,

    /// Example: `User(i32)`
    Unnamed,

    /// Example: `Guest`
    Unit,
}

/// Attributes specified with #[kinded(..)]
#[derive(Debug, Default)]
pub struct KindedAttributes {
    /// Name for the kind type, specified with `kind = ...`
    pub kind: Option<Ident>,

    /// Traits to derive, specified with `derive(...)`
    pub derive: Option<Vec<Path>>,
}
