mod kind_enum;
mod main_enum;

use crate::models::Meta;
use proc_macro2::TokenStream;
use quote::quote;

pub fn generate(meta: Meta) -> TokenStream {
    let kind_enum = kind_enum::gen_kind_enum(&meta);
    let main_enum_extra = main_enum::gen_main_enum_extra(&meta);

    quote!(
        #kind_enum
        #main_enum_extra
    )
}
