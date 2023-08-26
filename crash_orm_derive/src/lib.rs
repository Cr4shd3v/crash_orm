extern crate proc_macro;

use proc_macro::TokenStream;
use crate::entity_derive::derive_entity_impl;
use crate::schema_derive::derive_schema_impl;

mod entity_derive;
mod schema_derive;
mod util;

#[proc_macro_derive(Entity)]
pub fn derive_entity(input: TokenStream) -> TokenStream {
    derive_entity_impl(input)
}

#[proc_macro_derive(Schema)]
pub fn derive_schema(input: TokenStream) -> TokenStream {
    derive_schema_impl(input)
}