use proc_macro::*;
use quote::{quote, ToTokens};
use syn::Item;

pub fn entity_impl(input: TokenStream) -> TokenStream {
    let item: Item = syn::parse(input).unwrap();
    let struct_item = match item {
        Item::Struct(struct_item) => struct_item,
        _ => panic!("unsupported type for entity"),
    };

    let visibility = struct_item.vis;
    let ident = struct_item.ident;
    let fields = struct_item.fields;

    let mut fields_output = quote! {
        pub id: Option<u32>,
    };

    for field in fields {
        fields_output.extend(field.to_token_stream());
    }

    TokenStream::from(quote! {
        #[derive(crash_orm::crash_orm_derive::Loadable)]
        #visibility struct #ident {
            #fields_output
        }
    })
}