use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{Generics, Meta as SynMeta, Path, Visibility};

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
            .filter(|name| !self.kinded_attrs.should_skip_derive(name))
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

    pub fn main_enum_with_generics(&self) -> TokenStream {
        let type_name = &self.ident;
        let generics = &self.generics;

        quote!(#type_name #generics)
    }

    pub fn meta_attrs(&self) -> Vec<SynMeta> {
        self.kinded_attrs.meta_attrs.clone().unwrap_or_default()
    }
}

#[derive(Debug)]
pub struct Variant {
    pub ident: Ident,
    pub fields_type: FieldsType,
    /// Custom display/parse name specified with `#[kinded(rename = "...")]`.
    /// When set, this overrides the automatic case conversion for Display and FromStr.
    pub rename: Option<String>,
    /// Extra attributes to apply to the generated kind variant (e.g., `#[default]`, `#[serde(rename = "...")]`).
    pub attrs: Vec<SynMeta>,
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

    /// Opt out default derives/implementations for traits like Debug, Clone, Copy, PartialEq, Eq,
    /// FromStr, Display, etc. It may be needed in some cases for compatibility with other crates
    /// that provide similar macros. See https://github.com/greyblake/kinded/pull/19
    pub skip_derive: Option<Vec<Ident>>,

    /// Attributes to customize implementation for Display trait
    pub display: Option<DisplayCase>,

    /// Extra attributes to apply to the generated kind enum (e.g., `#[serde(rename_all = "camelCase")]`).
    pub meta_attrs: Option<Vec<SynMeta>>,
}

impl KindedAttributes {
    /// Check if a trait should be skipped from derive/implementation.
    pub fn should_skip_derive(&self, name: &str) -> bool {
        self.skip_derive
            .as_ref()
            .map(|traits| traits.iter().any(|t| t == name))
            .unwrap_or(false)
    }
}

/// This uses the same names as serde + "Title Case" variant.
/// Some names are different from what `convert_case` crate uses.
#[derive(Debug, Clone, Copy)]
pub enum DisplayCase {
    /// snake_case
    Snake,

    /// camelCase
    Camel,

    /// PascalCase
    Pascal,

    /// SCREAMING_SNAKE_CASE
    ScreamingSnake,

    /// kebab-case
    Kebab,

    /// SCREAMING-KEBAB-CASE
    ScreamingKebab,

    /// Title Case
    Title,

    /// lowercase
    Lower,

    /// UPPERCASE
    Upper,
}

impl From<DisplayCase> for convert_case::Case {
    fn from(display_case: DisplayCase) -> convert_case::Case {
        use convert_case::Case;

        // Note that convert_case use slightly different names than serde.
        match display_case {
            DisplayCase::Snake => Case::Snake,
            DisplayCase::Camel => Case::Camel,
            DisplayCase::Pascal => Case::Pascal,
            DisplayCase::ScreamingSnake => Case::ScreamingSnake,
            DisplayCase::Kebab => Case::Kebab,
            DisplayCase::ScreamingKebab => Case::Cobol,
            DisplayCase::Title => Case::Title,
            DisplayCase::Lower => Case::Flat,
            DisplayCase::Upper => Case::UpperFlat,
        }
    }
}

impl DisplayCase {
    pub fn all() -> impl Iterator<Item = Self> {
        use DisplayCase::*;
        [
            Snake,
            Camel,
            Pascal,
            ScreamingSnake,
            Kebab,
            ScreamingKebab,
            Title,
            Lower,
            Upper,
        ]
        .into_iter()
    }

    pub fn apply(self, s: &str) -> String {
        use convert_case::{Case, Casing};
        let case: Case = self.into();
        s.to_case(case)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    fn create_meta(kinded_attrs: KindedAttributes) -> Meta {
        Meta {
            vis: Visibility::Inherited,
            ident: format_ident!("TestEnum"),
            generics: Generics::default(),
            variants: vec![],
            kinded_attrs,
        }
    }

    #[test]
    fn derive_traits_default() {
        let meta = create_meta(KindedAttributes::default());
        let traits: Vec<String> = meta
            .derive_traits()
            .iter()
            .map(|p| quote!(#p).to_string())
            .collect();
        assert_eq!(traits, vec!["Debug", "Clone", "Copy", "PartialEq", "Eq"]);
    }

    #[test]
    fn derive_traits_with_extra() {
        let meta = create_meta(KindedAttributes {
            derive: Some(vec![parse_quote!(Hash), parse_quote!(Serialize)]),
            ..Default::default()
        });
        let traits: Vec<String> = meta
            .derive_traits()
            .iter()
            .map(|p| quote!(#p).to_string())
            .collect();
        assert_eq!(
            traits,
            vec![
                "Debug",
                "Clone",
                "Copy",
                "PartialEq",
                "Eq",
                "Hash",
                "Serialize"
            ]
        );
    }

    #[test]
    fn derive_traits_skip_one() {
        let meta = create_meta(KindedAttributes {
            skip_derive: Some(vec![format_ident!("Clone")]),
            ..Default::default()
        });
        let traits: Vec<String> = meta
            .derive_traits()
            .iter()
            .map(|p| quote!(#p).to_string())
            .collect();
        assert_eq!(traits, vec!["Debug", "Copy", "PartialEq", "Eq"]);
    }

    #[test]
    fn derive_traits_skip_multiple() {
        let meta = create_meta(KindedAttributes {
            skip_derive: Some(vec![
                format_ident!("Clone"),
                format_ident!("Copy"),
                format_ident!("Eq"),
            ]),
            ..Default::default()
        });
        let traits: Vec<String> = meta
            .derive_traits()
            .iter()
            .map(|p| quote!(#p).to_string())
            .collect();
        assert_eq!(traits, vec!["Debug", "PartialEq"]);
    }

    #[test]
    fn derive_traits_skip_all() {
        let meta = create_meta(KindedAttributes {
            skip_derive: Some(vec![
                format_ident!("Debug"),
                format_ident!("Clone"),
                format_ident!("Copy"),
                format_ident!("PartialEq"),
                format_ident!("Eq"),
            ]),
            ..Default::default()
        });
        let traits: Vec<String> = meta
            .derive_traits()
            .iter()
            .map(|p| quote!(#p).to_string())
            .collect();
        assert!(traits.is_empty());
    }

    #[test]
    fn derive_traits_skip_and_add() {
        let meta = create_meta(KindedAttributes {
            derive: Some(vec![parse_quote!(Hash)]),
            skip_derive: Some(vec![format_ident!("Clone"), format_ident!("Copy")]),
            ..Default::default()
        });
        let traits: Vec<String> = meta
            .derive_traits()
            .iter()
            .map(|p| quote!(#p).to_string())
            .collect();
        assert_eq!(traits, vec!["Debug", "PartialEq", "Eq", "Hash"]);
    }
}
