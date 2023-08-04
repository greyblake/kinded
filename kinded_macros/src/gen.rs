use crate::models::{FieldsType, Meta, Variant};
use proc_macro2::{Ident, TokenStream};
use quote::quote;

pub fn generate(meta: Meta) -> TokenStream {
    let kind_definition = gen_enum_kind(&meta);
    let fn_kind = gen_fn_kind(&meta);
    let type_name = &meta.ident;
    let kind_name = meta.kind_name();
    let generics = &meta.generics;

    let type_with_generics = quote!(#type_name #generics);

    quote!(
        #kind_definition

        impl #generics #type_with_generics {
            #fn_kind
        }

        impl #generics ::kinded::Kinded for #type_with_generics {
            type Kind = #kind_name;

            fn kind(&self) -> #kind_name {
                self.kind()
            }
        }

        // From<T>
        impl #generics From<#type_with_generics> for #kind_name {
            fn from(value: #type_with_generics) -> #kind_name {
                value.kind()
            }
        }

        // From<&T>
        impl #generics From<&#type_with_generics> for #kind_name {
            fn from(value: &#type_with_generics) -> #kind_name {
                value.kind()
            }
        }
    )
}

fn gen_enum_kind(meta: &Meta) -> TokenStream {
    let vis = &meta.vis;
    let kind_name = meta.kind_name();
    let variant_name_idents = meta.variants.iter().map(|v| &v.ident);
    let traits = meta.derive_traits();

    quote!(
        #[derive(#(#traits),*)]
        #vis enum #kind_name {
            #(#variant_name_idents),*
        }
    )
}

fn gen_fn_kind(meta: &Meta) -> TokenStream {
    let name = &meta.ident;
    let kind_name = meta.kind_name();
    let match_branches = meta
        .variants
        .iter()
        .map(|variant| gen_match_branch(name, &kind_name, variant));

    quote!(
        fn kind(&self) -> #kind_name {
            match self {
                #(#match_branches),*
            }
        }
    )
}

fn gen_match_branch(name: &Ident, kind_name: &Ident, variant: &Variant) -> TokenStream {
    let variant_name = &variant.ident;
    let variant_destruct = match variant.fields_type {
        FieldsType::Named => quote!({ .. }),
        FieldsType::Unnamed => quote!((..)),
        FieldsType::Unit => quote!(),
    };

    quote!(
        #name::#variant_name #variant_destruct => #kind_name::#variant_name
    )
}
