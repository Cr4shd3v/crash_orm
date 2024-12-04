use proc_macro::TokenStream;
use crate::crud_impl::crud_impl;

mod crud_impl;

#[proc_macro_derive(CRUD)]
pub fn crud(input: TokenStream) -> TokenStream {
    crud_impl(input)
}