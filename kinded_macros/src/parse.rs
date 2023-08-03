use crate::models::{FieldsType, Meta, Variant};
use syn::{Data, DeriveInput};

pub fn parse_derive_input(input: DeriveInput) -> Result<Meta, syn::Error> {
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
        variants: data.variants.iter().map(parse_variant).collect(),
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
