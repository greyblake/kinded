use crate::models::{FieldsType, Meta, Variant};
use proc_macro2::{Ident, TokenStream};
use quote::quote;

pub fn gen_main_enum_extra(meta: &Meta) -> TokenStream {
    let fn_kind = gen_fn_kind(meta);
    let main_enum_with_generics = meta.main_enum_with_generics();
    let generics = &meta.generics;

    let impl_kinded_trait = gen_impl_kinded_trait(meta);

    quote!(
        impl #generics #main_enum_with_generics {                              // impl<T> Drink<T> {
            #fn_kind                                                           //     fn kind(&self) -> DrinkKind { ... }
        }                                                                      // }

        #impl_kinded_trait                                                     // impl<T> ::kinded::Kinded for Drink<T> { .. }
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
        pub const fn kind(&self) -> #kind_name {                               // pub const fn kind(&self) -> DrinkKind {
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

fn gen_impl_kinded_trait(meta: &Meta) -> TokenStream {
    let kind_name = meta.kind_name();
    let main_enum_with_generics = meta.main_enum_with_generics();
    let generics = &meta.generics;

    quote!(
        impl #generics ::kinded::Kinded for #main_enum_with_generics {         // impl<T> ::kinded::Kinded for Drink<T> {
            type Kind = #kind_name;                                            //     type Kind = DrinkKind;
                                                                               //
            fn kind(&self) -> #kind_name {                                     //     fn kind(&self) -> DrinkKind {
                self.kind()                                                    //         self.kind()
            }                                                                  //     }
        }                                                                      // }
    )
}
