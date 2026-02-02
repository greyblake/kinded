use crate::models::{DisplayCase, FieldsType, KindedAttributes, Meta, Variant};
use proc_macro2::Ident;
use quote::ToTokens;
use syn::{
    Attribute, Data, DeriveInput, LitStr, Meta as SynMeta, Path, Token, bracketed, parenthesized,
    parse::{Parse, ParseStream},
    spanned::Spanned,
};

pub fn parse_derive_input(input: DeriveInput) -> Result<Meta, syn::Error> {
    let kinded_attrs: KindedAttributes = {
        match find_kinded_attr(&input)? {
            Some(kinded_attr) => syn::parse2(kinded_attr.to_token_stream())?,
            None => KindedAttributes::default(),
        }
    };

    let data = match input.data {
        Data::Enum(enum_data) => enum_data,
        Data::Struct(..) | Data::Union(..) => {
            return Err(syn::Error::new(
                input.ident.span(),
                "Kinded can be derived only on enums",
            ));
        }
    };

    Ok(Meta {
        vis: input.vis,
        ident: input.ident,
        generics: input.generics,
        variants: data.variants.iter().map(parse_variant).collect(),
        kinded_attrs,
    })
}

fn parse_variant(variant: &syn::Variant) -> Variant {
    let rename = find_variant_kinded_rename(&variant.attrs);
    let attrs = find_variant_kinded_attrs(&variant.attrs);
    Variant {
        ident: variant.ident.clone(),
        fields_type: parse_fields_type(&variant.fields),
        rename,
        attrs,
    }
}

/// Parsed variant-level #[kinded(...)] attributes
struct VariantKindedAttrs {
    rename: Option<String>,
    attrs: Vec<SynMeta>,
}

/// Parse all #[kinded(...)] attributes on a variant.
/// Handles combined attributes like #[kinded(rename = "...", attrs(...))]
fn parse_variant_kinded_attrs(attrs: &[Attribute]) -> VariantKindedAttrs {
    let mut result = VariantKindedAttrs {
        rename: None,
        attrs: Vec::new(),
    };

    for attr in attrs {
        if attr.path().is_ident("kinded") {
            let _ = attr.parse_args_with(|input: ParseStream| {
                while !input.is_empty() {
                    let attr_name: Ident = input.parse()?;

                    if attr_name == "rename" {
                        let _: Token!(=) = input.parse()?;
                        let lit_str: LitStr = input.parse()?;
                        result.rename = Some(lit_str.value());
                    } else if attr_name == "attrs" {
                        let content;
                        parenthesized!(content in input);
                        let parsed_attrs = content.parse_terminated(SynMeta::parse, Token![,])?;
                        result.attrs.extend(parsed_attrs);
                    }
                    // Ignore unknown attributes at variant level

                    // Parse `,` if not at end
                    if !input.is_empty() {
                        let _: Token![,] = input.parse()?;
                    }
                }
                Ok(())
            });
        }
    }

    result
}

/// Find `#[kinded(rename = "...")]` attribute on a variant and extract the rename value.
fn find_variant_kinded_rename(attrs: &[Attribute]) -> Option<String> {
    parse_variant_kinded_attrs(attrs).rename
}

/// Find `#[kinded(attrs(...))]` attribute on a variant and extract the attrs.
fn find_variant_kinded_attrs(attrs: &[Attribute]) -> Vec<SynMeta> {
    parse_variant_kinded_attrs(attrs).attrs
}

fn parse_fields_type(fields: &syn::Fields) -> FieldsType {
    match fields {
        syn::Fields::Named(..) => FieldsType::Named,
        syn::Fields::Unnamed(..) => FieldsType::Unnamed,
        syn::Fields::Unit => FieldsType::Unit,
    }
}

/// Find `#[kinded(..)]` attribute on the enum.
fn find_kinded_attr(input: &DeriveInput) -> Result<Option<&Attribute>, syn::Error> {
    let kinded_attrs: Vec<_> = input
        .attrs
        .iter()
        .filter(|&attr| attr.path().is_ident("kinded"))
        .collect();

    if kinded_attrs.len() > 1 {
        let &attr = kinded_attrs.last().unwrap();
        let span = attr.span();
        let msg = "Multiple #[kinded(..)] attributes are not allowed.";
        Err(syn::Error::new(span, msg))
    } else {
        let maybe_kinded_attr = kinded_attrs.into_iter().next();
        Ok(maybe_kinded_attr)
    }
}

impl Parse for KindedAttributes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut kinded_attrs = KindedAttributes::default();

        // Unwrap the irrelevant part and reassign input to the relevant input:
        //
        //     #[kinded(  RELEVANT_INPUT  )]
        //
        let input = {
            let _: Token!(#) = input.parse()?;
            let bracketed_content;
            bracketed!(bracketed_content in input);
            let _kinded: Ident = bracketed_content.parse()?;

            let parenthesized_content;
            parenthesized!(parenthesized_content in bracketed_content);
            parenthesized_content
        };

        while !input.is_empty() {
            let attr_name: Ident = input.parse()?;
            if attr_name == "kind" {
                let _: Token!(=) = input.parse()?;
                let kind: Ident = input.parse()?;
                if kinded_attrs.kind.is_none() {
                    kinded_attrs.kind = Some(kind);
                } else {
                    let msg = format!("Duplicated attribute: {attr_name}");
                    return Err(syn::Error::new(attr_name.span(), msg));
                }
            } else if attr_name == "derive" {
                let derive_input;
                parenthesized!(derive_input in input);
                let parsed_traits = derive_input.parse_terminated(Path::parse, Token![,])?;
                let traits: Vec<Path> = parsed_traits.into_iter().collect();
                if kinded_attrs.derive.is_none() {
                    kinded_attrs.derive = Some(traits);
                } else {
                    let msg = format!("Duplicated attribute: {attr_name}");
                    return Err(syn::Error::new(attr_name.span(), msg));
                }
            } else if attr_name == "skip_derive" {
                let skip_input;
                parenthesized!(skip_input in input);
                let parsed_traits = skip_input.parse_terminated(Ident::parse, Token![,])?;
                let traits: Vec<Ident> = parsed_traits.into_iter().collect();

                // Validate that only allowed traits are specified
                const ALLOWED_SKIP_DERIVE: &[&str] = &[
                    "Debug",
                    "Clone",
                    "Copy",
                    "PartialEq",
                    "Eq",
                    "Display",
                    "FromStr",
                    "From",
                ];
                for trait_name in &traits {
                    if !ALLOWED_SKIP_DERIVE.contains(&trait_name.to_string().as_str()) {
                        let msg = format!(
                            "Unknown trait to skip: `{trait_name}`. Allowed traits: {}",
                            ALLOWED_SKIP_DERIVE.join(", ")
                        );
                        return Err(syn::Error::new(trait_name.span(), msg));
                    }
                }

                if kinded_attrs.skip_derive.is_none() {
                    kinded_attrs.skip_derive = Some(traits);
                } else {
                    let msg = format!("Duplicated attribute: {attr_name}");
                    return Err(syn::Error::new(attr_name.span(), msg));
                }
            } else if attr_name == "display" {
                let _: Token!(=) = input.parse()?;
                let case_lit_str: LitStr = input.parse()?;
                let case = match case_lit_str.value().as_ref() {
                    "snake_case" => DisplayCase::Snake,
                    "camelCase" => DisplayCase::Camel,
                    "PascalCase" => DisplayCase::Pascal,
                    "SCREAMING_SNAKE_CASE" => DisplayCase::ScreamingSnake,
                    "kebab-case" => DisplayCase::Kebab,
                    "SCREAMING-KEBAB-CASE" => DisplayCase::ScreamingKebab,
                    "Title Case" => DisplayCase::Title,
                    "lowercase" => DisplayCase::Lower,
                    "UPPERCASE" => DisplayCase::Upper,
                    _ => {
                        let valid_values = [
                            "snake_case",
                            "camelCase",
                            "PascalCase",
                            "SCREAMING_SNAKE_CASE",
                            "kebab-case",
                            "SCREAMING-KEBAB-CASE",
                            "Title Case",
                            "lowercase",
                            "UPPERCASE",
                        ]
                        .map(|value| format!(r#""{value}""#))
                        .join(", ");
                        let given_value = format!(r#""{}""#, case_lit_str.value());
                        let msg = format!(
                            "Invalid value for display: {given_value}\nValid values are: {valid_values}"
                        );
                        return Err(syn::Error::new(case_lit_str.span(), msg));
                    }
                };
                if kinded_attrs.derive.is_none() {
                    kinded_attrs.display = Some(case);
                } else {
                    let msg = format!("Duplicated attribute: {attr_name}");
                    return Err(syn::Error::new(attr_name.span(), msg));
                }
            } else if attr_name == "attrs" {
                let derive_input;
                parenthesized!(derive_input in input);

                let parsed_attr = derive_input.parse_terminated(SynMeta::parse, Token![,])?;
                kinded_attrs.meta_attrs = Some(parsed_attr.into_iter().collect());
            } else {
                let msg = format!("Unknown attribute: {attr_name}");
                return Err(syn::Error::new(attr_name.span(), msg));
            }

            // Parse `,` unless it's the end of the stream
            if !input.is_empty() {
                let _comma: Token![,] = input.parse()?;
            }
        }

        Ok(kinded_attrs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;

    fn parse_kinded_attrs(tokens: proc_macro2::TokenStream) -> syn::Result<KindedAttributes> {
        syn::parse2(tokens)
    }

    #[test]
    fn parse_skip_derive_single() {
        let attrs = parse_kinded_attrs(quote! { #[kinded(skip_derive(Clone))] }).unwrap();
        let skip: Vec<String> = attrs
            .skip_derive
            .unwrap()
            .iter()
            .map(|i| i.to_string())
            .collect();
        assert_eq!(skip, vec!["Clone"]);
    }

    #[test]
    fn parse_skip_derive_multiple() {
        let attrs =
            parse_kinded_attrs(quote! { #[kinded(skip_derive(Clone, Copy, Debug))] }).unwrap();
        let skip: Vec<String> = attrs
            .skip_derive
            .unwrap()
            .iter()
            .map(|i| i.to_string())
            .collect();
        assert_eq!(skip, vec!["Clone", "Copy", "Debug"]);
    }

    #[test]
    fn parse_skip_derive_all_allowed() {
        let attrs = parse_kinded_attrs(quote! {
            #[kinded(skip_derive(Debug, Clone, Copy, PartialEq, Eq, Display, FromStr, From))]
        })
        .unwrap();
        let skip: Vec<String> = attrs
            .skip_derive
            .unwrap()
            .iter()
            .map(|i| i.to_string())
            .collect();
        assert_eq!(
            skip,
            vec![
                "Debug",
                "Clone",
                "Copy",
                "PartialEq",
                "Eq",
                "Display",
                "FromStr",
                "From"
            ]
        );
    }

    #[test]
    fn parse_skip_derive_with_other_attrs() {
        let attrs = parse_kinded_attrs(quote! {
            #[kinded(kind = MyKind, skip_derive(Clone, Copy), derive(Hash))]
        })
        .unwrap();

        assert_eq!(attrs.kind.unwrap().to_string(), "MyKind");

        let skip: Vec<String> = attrs
            .skip_derive
            .unwrap()
            .iter()
            .map(|i| i.to_string())
            .collect();
        assert_eq!(skip, vec!["Clone", "Copy"]);

        let derive: Vec<String> = attrs
            .derive
            .unwrap()
            .iter()
            .map(|p| quote!(#p).to_string())
            .collect();
        assert_eq!(derive, vec!["Hash"]);
    }

    #[test]
    fn parse_skip_derive_invalid_trait() {
        let result = parse_kinded_attrs(quote! { #[kinded(skip_derive(Hash))] });
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("Unknown trait to skip"));
        assert!(err.contains("Hash"));
    }

    #[test]
    fn parse_skip_derive_duplicated() {
        let result = parse_kinded_attrs(quote! {
            #[kinded(skip_derive(Clone), skip_derive(Copy))]
        });
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("Duplicated attribute"));
    }
}
