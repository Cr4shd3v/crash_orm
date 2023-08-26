use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Ident, parse_macro_input};
use crate::util::extract_type_from_option;

pub fn derive_query_impl(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    let Data::Struct(struct_data) = derive_input.data else {
        panic!("only structs are supported")
    };

    let original_ident = derive_input.ident;
    let vis = derive_input.vis;

    let mut column_consts = quote!();

    for field in struct_data.fields {
        let field_ident = field.ident.unwrap();
        let field_ident_str = field_ident.to_string();
        let field_ident_upper = Ident::new(&*field_ident_str.to_uppercase(), field_ident.span());
        let field_type = if &*field_ident_str == "id" {
            extract_type_from_option(&field.ty).unwrap_or(field.ty)
        } else { field.ty };

        column_consts.extend(quote! {
            pub const #field_ident_upper: crash_orm::EntityColumn::<#field_type, #original_ident> = crash_orm::EntityColumn::<#field_type, #original_ident>::new(#field_ident_str);
        });
    }

    let ident = Ident::new(&*format!("{}Column", original_ident.to_string()), original_ident.span());

    let output = quote! {
        #vis struct #ident;

        impl #ident {
            #column_consts
        }

        #[crash_orm::async_trait::async_trait]
        impl crash_orm::QueryEntity<#original_ident> for #original_ident {}
    };

    output.into()
}