use crate::models::{DisplayCase, FieldsType, KindedAttributes, Meta, Variant};
use proc_macro2::Ident;
use quote::ToTokens;
use syn::{
    bracketed, parenthesized,
    parse::{Parse, ParseStream},
    spanned::Spanned,
    Attribute, Data, DeriveInput, LitStr, Path, Token, Meta as SynMeta,
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
    Variant {
        ident: variant.ident.clone(),
        fields_type: parse_fields_type(&variant.fields),
    }
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
                        let msg = format!("Invalid value for display: {given_value}\nValid values are: {valid_values}");
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
                kinded_attrs.attr = Some(parsed_attr.into_iter().collect());
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
