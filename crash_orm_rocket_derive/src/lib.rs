use crate::crud_impl::crud_impl;
use proc_macro::TokenStream;

mod crud_impl;

#[cfg(feature = "crud")]
#[proc_macro_derive(CRUD)]
pub fn crud(input: TokenStream) -> TokenStream {
    crud_impl(input)
}