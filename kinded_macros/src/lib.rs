//! # Kinded
//!
//! Generate Rust enum kind types without boilerplate.
//!
//! Author: [Serhii Potapov](https://www.greyblake.com/)
//!
//! This is a supporting macro crate, that should not be used directly.
//! For the documentation please refer to [kinded](https://docs.rs/kinded/) crate.

pub(crate) mod generate;
pub(crate) mod models;
pub(crate) mod parse;

use proc_macro2::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(Kinded, attributes(kinded))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    expand_derive(input)
        .unwrap_or_else(|e| syn::Error::to_compile_error(&e))
        .into()
}

fn expand_derive(input: proc_macro::TokenStream) -> Result<TokenStream, syn::Error> {
    let derive_input: DeriveInput =
        syn::parse(input).expect("kinded failed parse token stream as DeriveInput");
    let meta = parse::parse_derive_input(derive_input)?;
    Ok(generate::generate(meta))
}
