use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, parse_macro_input};

pub fn derive_loadable_impl(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    let Data::Struct(struct_data) = derive_input.data else {
        panic!("only structs are supported")
    };

    let ident = derive_input.ident;

    let mut fields = quote!();

    let mut index = 0usize;
    for field in struct_data.fields {
        let ident = field.ident.unwrap();

        fields.extend(quote! {
            #ident: row.get(#index),
        });

        index += 1;
    }

    let output = quote! {
        impl crash_orm::Loadable for #ident {
            type Output = #ident;

            fn load_from_row(row: crash_orm::tokio_postgres::Row) -> #ident {
                #ident {
                    #fields
                }
            }
        }
    };

    output.into()
}