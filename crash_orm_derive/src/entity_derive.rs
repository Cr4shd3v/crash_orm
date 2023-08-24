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

    let mut select_fields = quote!();
    let mut all_field_names = quote!();
    let mut all_field_self_values = quote!();
    let mut all_field_self_values_format = String::new();
    let mut insert_field_names = quote!();
    let mut insert_field_self_values = quote!();
    let mut insert_field_self_values_format = String::new();

    let mut all_index = 0usize;
    let mut insert_index = 0usize;
    for field in struct_data.fields {
        let ident = field.ident.unwrap();

        select_fields.extend(quote! {
            #ident: row.get(#all_index),
        });

        all_field_names.extend(quote! {
            #ident,
        });

        all_field_self_values.extend(quote! {
            &self.#ident,
        });

        if ident.to_string() != "id" {
            insert_field_names.extend(quote! {
                #ident,
            });

            insert_field_self_values.extend(quote! {
                &self.#ident,
            });

            insert_index += 1;

            insert_field_self_values_format.push_str(&*format!("${},", all_index));
        }

        all_index += 1;

        all_field_self_values_format.push_str(&*format!("${},", all_index));
    }

    let all_field_names = all_field_names.to_string();
    let all_field_names = all_field_names.strip_suffix(",").unwrap();
    let all_field_self_values_format = all_field_self_values_format.strip_suffix(",").unwrap();
    let insert_field_names = insert_field_names.to_string();
    let insert_field_names = insert_field_names.strip_suffix(",").unwrap();
    let insert_field_self_values_format = insert_field_self_values_format.strip_suffix(",").unwrap();

    let output = quote! {
        #[crash_orm::async_trait::async_trait]
        impl crash_orm::Entity for #ident {
            type Output = #ident;

            fn load_from_row(row: crash_orm::tokio_postgres::Row) -> #ident {
                #ident {
                    #select_fields
                }
            }

            async fn get_by_id(connection: &crash_orm::DatabaseConnection, id: u32) -> Result<#ident, crash_orm::tokio_postgres::Error> {
                let row = connection.query_one(&*format!("SELECT * FROM {} WHERE id = $1", #ident_str), &[&id]).await?;
                Ok(Self::load_from_row(row))
            }

            async fn insert(&self, connection: &crash_orm::DatabaseConnection) -> Result<u32, crash_orm::tokio_postgres::Error> {
                let row = connection.query(
                    &*format!("INSERT INTO {}({}) VALUES ({}) RETURNING id", #ident_str, #insert_field_names, #insert_field_self_values_format),
                    &[#insert_field_self_values]
                ).await?;
                Ok(row.get(0).unwrap().get(0))
            }

            async fn insert_set_id(&mut self, connection: &crash_orm::DatabaseConnection) -> Result<u32, crash_orm::tokio_postgres::Error> {
                let id = self.insert(connection).await?;
                self.id = id;
                Ok(id)
            }
        }
    };

    output.into()
}