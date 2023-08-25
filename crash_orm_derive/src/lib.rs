extern crate proc_macro;

use proc_macro::TokenStream;
use crate::entity_derive::derive_loadable_impl;

mod entity_derive;

#[proc_macro_derive(Entity)]
pub fn derive_loadable(input: TokenStream) -> TokenStream {
    derive_loadable_impl(input)
}