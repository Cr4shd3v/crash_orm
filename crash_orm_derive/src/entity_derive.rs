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

        if ident.to_string() != "id" {
            insert_field_names.extend(quote! {
                #ident,
            });

            insert_field_self_values.extend(quote! {
                &self.#ident,
            });

            insert_index += 1;

            insert_field_self_values_format.push_str(&*format!("${},", insert_index));
        }

        all_index += 1;

        all_field_self_values_format.push_str(&*format!("${},", all_index));
    }

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

            async fn insert_get_id(&self, connection: &crash_orm::DatabaseConnection) -> Result<u32, crash_orm::tokio_postgres::Error> {
                let row = connection.query(
                    &*format!("INSERT INTO {}({}) VALUES ({}) RETURNING id", #ident_str, #insert_field_names, #insert_field_self_values_format),
                    &[#insert_field_self_values]
                ).await?;
                Ok(row.get(0).unwrap().get(0))
            }

            async fn insert_set_id(&mut self, connection: &crash_orm::DatabaseConnection) -> Result<(), crash_orm::tokio_postgres::Error> {
                let id = self.insert_get_id(connection).await?;
                self.id = Some(id);
                Ok(())
            }

            async fn remove(&mut self, connection: &crash_orm::DatabaseConnection) -> Result<(), crash_orm::tokio_postgres::Error> {
                if self.id.is_none() {
                    return Ok(());
                }

                connection.execute(&*format!("DELETE FROM {} WHERE id = $1", #ident_str), &[&self.id]).await?;
                self.id = None;
                Ok(())
            }

            async fn persist(&mut self, connection: &crash_orm::DatabaseConnection) -> Result<(), crash_orm::tokio_postgres::Error> {
                if self.id.is_none() {
                    self.insert_set_id(connection).await
                } else {
                    panic!("Not supported yet");
                }
            }
        }
    };

    output.into()
}