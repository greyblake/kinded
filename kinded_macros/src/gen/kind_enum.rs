use crate::models::{DisplayCase, Meta};
use proc_macro2::{Ident, TokenStream};
use quote::quote;

pub fn gen_kind_enum(meta: &Meta) -> TokenStream {
    let kind_enum_definition = gen_definition(meta);
    let impl_from_traits = gen_impl_from_traits(meta);
    let impl_display_trait = gen_impl_display_trait(meta);
    let impl_from_str_trait = gen_impl_from_str_trait(meta);

    quote!(
        #kind_enum_definition
        #impl_from_traits
        #impl_display_trait
        #impl_from_str_trait
    )
}

fn gen_definition(meta: &Meta) -> TokenStream {
    let vis = &meta.vis;
    let kind_name = meta.kind_name();
    let variant_names: Vec<&Ident> = meta.variants.iter().map(|v| &v.ident).collect();
    let traits = meta.derive_traits();

    quote!(
        #[derive(#(#traits),*)]                                                // #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        #vis enum #kind_name {                                                 // pub enum DrinkKind {
            #(#variant_names),*                                                //     Mate, Coffee, Tea
        }                                                                      // }

        impl #kind_name {                                                      // impl DrinkKind {
            pub fn all() -> impl Iterator<Item = #kind_name> {                 //     pub fn all() -> impl Iterator<Item = DrinkKind> {
                [                                                              //         [
                    #(#kind_name::#variant_names),*                            //             DrinkKind::Mate, DrinkKind::Coffee, DrinkKind::Tea
                ].into_iter()                                                  //         ]
            }                                                                  //     }
        }                                                                      // }
    )
}

fn gen_impl_from_traits(meta: &Meta) -> TokenStream {
    let kind_name = meta.kind_name();
    let generics = &meta.generics;
    let main_enum_with_generics = meta.main_enum_with_generics();

    quote!(
        impl #generics From<#main_enum_with_generics> for #kind_name {         // impl<T> From<Drink<T>> for DrinkKind {
            fn from(value: #main_enum_with_generics) -> #kind_name {           //     fn from(value: Drink<T>) -> DrinkKind {
                value.kind()                                                   //         value.kind()
            }                                                                  //     }
        }                                                                      // }

        impl #generics From<&#main_enum_with_generics> for #kind_name {        // impl<T> From<Drink<T>> for DrinkKind {
            fn from(value: &#main_enum_with_generics) -> #kind_name {          //     fn from(value: &Drink<T>) -> DrinkKind {
                value.kind()                                                   //         value.kind()
            }                                                                  //     }
        }                                                                      // }
    )
}

fn gen_impl_display_trait(meta: &Meta) -> TokenStream {
    let kind_name = meta.kind_name();
    let maybe_case = meta.kinded_attrs.display;

    let match_branches = meta.variants.iter().map(|variant| {
        let original_variant_name_str = variant.ident.to_string();
        let cased_variant_name = apply_display_case(original_variant_name_str, maybe_case);
        let variant_name = &variant.ident;
        quote!(
            #kind_name::#variant_name => write!(f, #cased_variant_name)
        )
    });

    quote!(
        impl std::fmt::Display for #kind_name {                                     // impl std::fmt::Display for DrinkKind {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {    //     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {                                                        //         match self {
                    #(#match_branches),*                                            //             DrinkKind::Mate => write!(f, "mate"),
                }                                                                   //         }
            }                                                                       //     }
        }                                                                           //
    )
}

fn apply_display_case(original: String, maybe_display_case: Option<DisplayCase>) -> String {
    use convert_case::{Case, Casing};

    if let Some(display_case) = maybe_display_case {
        let case: Case = display_case.into();
        original.to_case(case)
    } else {
        original
    }
}

fn gen_impl_from_str_trait(meta: &Meta) -> TokenStream {
    let kind_name = meta.kind_name();

    let original_match_branches = meta.variants.iter().map(|variant| {
        let ident = &variant.ident;
        let name_str = ident.to_string();
        quote!(#name_str => Ok(#kind_name::#ident),)
    });

    quote!(
        impl ::core::str::FromStr for #kind_name {
            type Err = ::kinded::ParseKindError;

            fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
                match s {
                    #(#original_match_branches)*
                    _ => {
                        let type_name: String = std::any::type_name::<#kind_name>().to_owned();
                        let error = ::kinded::ParseKindError::from_type_name_and_string(type_name, s.to_owned());
                        Err(error)
                    },
                }
            }
        }
    )
}
