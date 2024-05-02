extern crate proc_macro;

use crate::entity_derive::derive_entity_impl;
use crate::schema_derive::derive_schema_impl;
use proc_macro::TokenStream;

mod entity_derive;
mod schema_derive;
mod util;

#[proc_macro_derive(Entity, attributes(mapped_by, primary_key))]
pub fn derive_entity(input: TokenStream) -> TokenStream {
    derive_entity_impl(input)
}

#[proc_macro_derive(Schema, attributes(primary_key))]
pub fn derive_schema(input: TokenStream) -> TokenStream {
    derive_schema_impl(input)
}
