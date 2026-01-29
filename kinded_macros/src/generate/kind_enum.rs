use crate::models::{DisplayCase, Meta};
use proc_macro2::{Ident, TokenStream};
use quote::quote;

pub fn gen_kind_enum(meta: &Meta) -> TokenStream {
    let kind_enum_definition = gen_definition(meta);
    let impl_from_traits = gen_impl_from_traits(meta);
    let impl_display_trait = gen_impl_display_trait(meta);
    let impl_from_str_trait = gen_impl_from_str_trait(meta);
    let impl_kind_trait = gen_impl_kind_trait(meta);

    quote!(
        #kind_enum_definition
        #impl_from_traits
        #impl_display_trait
        #impl_from_str_trait
        #impl_kind_trait
    )
}

fn gen_definition(meta: &Meta) -> TokenStream {
    let vis = &meta.vis;
    let kind_name = meta.kind_name();
    let variant_names: Vec<&Ident> = meta.variants.iter().map(|v| &v.ident).collect();
    let traits = meta.derive_traits();
    let attrs = meta.meta_attrs();

    quote!(
        #[derive(#(#traits),*)]                                                // #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        #(#[#attrs])*                                                          // #[serde(rename_all = "camelCase")]
        #vis enum #kind_name {                                                 // pub enum DrinkKind {
            #(#variant_names),*                                                //     Mate, Coffee, Tea
        }                                                                      // }

        impl #kind_name {                                                      // impl DrinkKind {
            pub fn all() -> &'static [#kind_name] {                            //     pub fn all() -> &'static [DrinkKind] {
                &[                                                             //         &[
                    #(#kind_name::#variant_names),*                            //             DrinkKind::Mate, DrinkKind::Coffee, DrinkKind::Tea
                ]                                                              //         ]
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
        // Use custom rename if specified, otherwise apply case conversion
        let display_name = if let Some(ref rename) = variant.rename {
            rename.clone()
        } else {
            let original_variant_name_str = variant.ident.to_string();
            apply_maybe_case(original_variant_name_str, maybe_case)
        };
        let variant_name = &variant.ident;
        quote!(
            #kind_name::#variant_name => write!(f, #display_name)
        )
    });

    quote!(
        impl core::fmt::Display for #kind_name {                                    // impl core::fmt::Display for DrinkKind {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {  //     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                match self {                                                        //         match self {
                    #(#match_branches),*                                            //             DrinkKind::Mate => write!(f, "mate"),
                }                                                                   //         }
            }                                                                       //     }
        }                                                                           //
    )
}

fn apply_maybe_case(original: String, maybe_display_case: Option<DisplayCase>) -> String {
    if let Some(display_case) = maybe_display_case {
        display_case.apply(&original)
    } else {
        original
    }
}

fn gen_impl_from_str_trait(meta: &Meta) -> TokenStream {
    let kind_name = meta.kind_name();

    // First priority: match custom renames (if any variant has a rename)
    let rename_match_branches: Vec<_> = meta
        .variants
        .iter()
        .filter_map(|variant| {
            variant.rename.as_ref().map(|rename| {
                let ident = &variant.ident;
                quote!(#rename => return Ok(#kind_name::#ident),)
            })
        })
        .collect();

    let original_match_branches = meta.variants.iter().map(|variant| {
        let ident = &variant.ident;
        let name_str = ident.to_string();
        quote!(#name_str => return Ok(#kind_name::#ident),)
    });

    let alt_match_branches = meta.variants.iter().map(|variant| {
        let ident = &variant.ident;
        let name_str = ident.to_string();
        let alternatives = DisplayCase::all().map(|case| case.apply(&name_str));
        quote!(#(#alternatives)|* => return Ok(#kind_name::#ident),)
    });

    // Only generate the rename match block if there are any renames
    let rename_match_block = if rename_match_branches.is_empty() {
        quote!()
    } else {
        quote!(
            // First try to match custom renames
            match s {
                #(#rename_match_branches)*
                _ => ()
            }
        )
    };

    quote!(
        impl ::core::str::FromStr for #kind_name {
            type Err = ::kinded::ParseKindError;

            fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
                #rename_match_block

                // Try to match the variants as they are (original names)
                match s {                                                      // match s {
                    #(#original_match_branches)*                               //     "HotMate" => Mate::HotMate,
                    _ => ()                                                    //      _ => (),
                }                                                              //

                // Now try to match all possible alternative spelling of
                // the variants
                match s {                                                      // match s {
                    #(#alt_match_branches)*                                    //     "hot_mate" | "HOT_MATE" | "hotMate" | .. => Mate::HotMate
                    _ => ()                                                    //      _ => ()
                }                                                              // }

                // If still no success, then return an error
                extern crate alloc;
                use alloc::borrow::ToOwned;
                let error = ::kinded::ParseKindError::from_type_and_string::<#kind_name>(s.to_owned());
                Err(error)
            }
        }
    )
}

fn gen_impl_kind_trait(meta: &Meta) -> TokenStream {
    let kind_name = meta.kind_name();

    quote!(
        impl ::kinded::Kind for #kind_name {
            fn all() -> &'static [#kind_name] {
                Self::all()
            }
        }
    )
}
