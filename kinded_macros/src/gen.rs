use crate::models::{FieldsType, Meta, Variant};
use proc_macro2::{Ident, TokenStream};
use quote::quote;

pub fn generate(meta: Meta) -> TokenStream {
    let enum_kind = gen_enum_kind(&meta);
    let impl_display_for_enum_kind = gen_impl_display_for_enum_kind(&meta);

    let fn_kind = gen_fn_kind(&meta);
    let type_name = &meta.ident;
    let kind_name = meta.kind_name();
    let generics = &meta.generics;

    let type_with_generics = quote!(#type_name #generics);

    quote!(
        #enum_kind                                                             // enum DrinkKind { Mate, Coffee, Tea }

        #impl_display_for_enum_kind                                            // impl std::fmt::Display for DrinkKind { ... }

        impl #generics #type_with_generics {                                   // impl<T> Drink<T> {
            #fn_kind                                                           //     fn kind(&self) -> DrinkKind { ... }
        }                                                                      // }

        impl #generics ::kinded::Kinded for #type_with_generics {              // impl<T> ::kinded::Kinded for Drink<T> {
            type Kind = #kind_name;                                            //     type Kind = DrinkKind;
                                                                               //
            fn kind(&self) -> #kind_name {                                     //     fn kind(&self) -> DrinkKind {
                self.kind()                                                    //         self.kind()
            }                                                                  //     }
        }                                                                      // }

        impl #generics From<#type_with_generics> for #kind_name {              // impl<'a, T> From<Drink<'a, T>> for DrinkKind {
            fn from(value: #type_with_generics) -> #kind_name {                //     fn from(value: Drink<'a, T>) -> DrinkKind {
                value.kind()                                                   //         value.kind()
            }                                                                  //     }
        }                                                                      // }

        impl #generics From<&#type_with_generics> for #kind_name {             // impl<'a, T> From<Drink<'a, T>> for DrinkKind {
            fn from(value: &#type_with_generics) -> #kind_name {               //     fn from(value: &Drink<'a, T>) -> DrinkKind {
                value.kind()                                                   //         value.kind()
            }                                                                  //     }
        }                                                                      // }
    )
}

fn gen_enum_kind(meta: &Meta) -> TokenStream {
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

fn gen_impl_display_for_enum_kind(meta: &Meta) -> TokenStream {
    let kind_name = meta.kind_name();
    let match_branches = meta.variants.iter().map(|variant| {
        let variant_name_str = variant.ident.to_string();
        let variant_name = &variant.ident;
        quote!(
            #kind_name::#variant_name => write!(f, #variant_name_str)
        )
    });

    quote!(
        impl std::fmt::Display for #kind_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #(#match_branches),*
                }
            }
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
        pub fn kind(&self) -> #kind_name {                                     // pub fn kind(&self) -> DrinkKind {
            match self {                                                       //     match self {
                #(#match_branches),*                                           //         Drink::Coffee(..) => DrinkKind::Coffee,
            }                                                                  //     }
        }                                                                      // }
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
