#![allow(unused_imports)]

extern crate proc_macro;
extern crate syn;

use proc_macro::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{parse_macro_input, DeriveInput, Expr, ItemFn, ItemStruct};

mod alohomora_type;

#[proc_macro_derive(AlohomoraType, attributes(out_type))] 
pub fn derive_alohomora_type(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    alohomora_type::derive_alohomora_ty_impl(input).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        //placeholder
        assert_eq!(true, true);
    }
}
