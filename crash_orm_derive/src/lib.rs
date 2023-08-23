extern crate proc_macro;

use proc_macro::TokenStream;
use crate::entity::entity_impl;
use crate::entity_derive::derive_loadable_impl;

mod entity;
mod entity_derive;

#[proc_macro_attribute]
pub fn entity(_args: TokenStream, input: TokenStream) -> TokenStream {
    entity_impl(input)
}

#[proc_macro_derive(Loadable)]
pub fn derive_loadable(input: TokenStream) -> TokenStream {
    derive_loadable_impl(input)
}