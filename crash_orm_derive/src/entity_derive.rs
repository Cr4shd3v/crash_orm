use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, parse_macro_input};

pub fn derive_loadable_impl(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    let Data::Struct(struct_data) = derive_input.data else {
        panic!("only structs are supported")
    };

    let ident = derive_input.ident;
    let ident_str = ident.to_string();

    let mut fields = quote!();
    let mut field_names = quote!();
    let mut field_self_values = quote!();
    let mut field_self_values_format = String::new();

    let mut index = 0usize;
    for field in struct_data.fields {
        let ident = field.ident.unwrap();

        fields.extend(quote! {
            #ident: row.get(#index),
        });

        field_names.extend(quote! {
            #ident,
        });

        field_self_values.extend(quote! {
            self.#ident,
        });

        field_self_values_format.push_str("{},");

        index += 1;
    }

    let field_names = field_names.to_string();
    let field_names = field_names.strip_suffix(",").unwrap();
    let field_self_values_format = field_self_values_format.strip_suffix(",").unwrap();

    let output = quote! {
        impl crash_orm::Entity for #ident {
            type Output = #ident;

            fn load_from_row(row: crash_orm::tokio_postgres::Row) -> #ident {
                #ident {
                    #fields
                }
            }

            fn get_select_query(&self) -> String {
                format!("SELECT * FROM {} WHERE id = {}", #ident_str, self.id)
            }

            fn get_insert_stmt(&self) -> String {
                format!("INSERT INTO {}({}) VALUES ({})", #ident_str, #field_names, format!(#field_self_values_format, #field_self_values))
            }
        }
    };

    output.into()
}