extern crate proc_macro;

use proc_macro::TokenStream;
use crate::entity_derive::derive_entity_impl;
use crate::query_derive::derive_query_impl;
use crate::schema_derive::derive_schema_impl;

mod entity_derive;
mod schema_derive;
mod query_derive;
mod util;

#[proc_macro_derive(Entity)]
pub fn derive_entity(input: TokenStream) -> TokenStream {
    derive_entity_impl(input)
}

#[proc_macro_derive(Schema)]
pub fn derive_schema(input: TokenStream) -> TokenStream {
    derive_schema_impl(input)
}

#[proc_macro_derive(Query)]
pub fn derive_query(input: TokenStream) -> TokenStream {
    derive_query_impl(input)
}