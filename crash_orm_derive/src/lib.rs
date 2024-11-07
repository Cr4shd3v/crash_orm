extern crate proc_macro;

use crate::entity_derive::derive_entity_impl;
use crate::result_mapping_derive::derive_result_mapping_impl;
use crate::schema_derive::derive_schema_impl;
use proc_macro::TokenStream;

mod entity_derive;
mod schema_derive;
mod util;
mod result_mapping_derive;

#[proc_macro_derive(Entity, attributes(mapped_by, primary_key))]
pub fn derive_entity(input: TokenStream) -> TokenStream {
    let mut output = derive_entity_impl(input.clone());
    output.extend(derive_result_mapping_impl(input));
    output
}

#[proc_macro_derive(Schema, attributes(primary_key))]
pub fn derive_schema(input: TokenStream) -> TokenStream {
    derive_schema_impl(input)
}

#[proc_macro_derive(ResultMapping)]
pub fn derive_result_mapping(input: TokenStream) -> TokenStream {
    derive_result_mapping_impl(input)
}
