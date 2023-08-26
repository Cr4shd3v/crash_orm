use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, parse_macro_input};

pub fn derive_entity_impl(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    let Data::Struct(struct_data) = derive_input.data else {
        panic!("only structs are supported")
    };

    let ident = derive_input.ident;
    let ident_str = ident.to_string().to_lowercase();

    let mut select_fields = quote!();
    let mut all_field_self_values_format = String::new();
    let mut insert_field_names = quote!();
    let mut insert_field_self_values = quote!();
    let mut insert_field_self_values_format = String::new();
    let mut update_fields = String::new();

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

            update_fields.push_str(&*format!("{} = ${}", ident.to_string(), insert_index));
            insert_field_self_values_format.push_str(&*format!("${},", insert_index));
        }

        all_index += 1;

        all_field_self_values_format.push_str(&*format!("${},", all_index));
    }

    insert_index += 1;

    let insert_field_names = insert_field_names.to_string();
    let insert_field_names = insert_field_names.strip_suffix(",").unwrap();
    let insert_field_self_values_format = insert_field_self_values_format.strip_suffix(",").unwrap();

    let select_by_id_string = format!("SELECT * FROM {} WHERE id = $1", ident_str);
    let select_all_string = format!("SELECT * FROM {}", ident_str);
    let insert_string = format!("INSERT INTO {}({}) VALUES ({}) RETURNING id", ident_str, insert_field_names, insert_field_self_values_format);
    let delete_string = format!("DELETE FROM {} WHERE id = $1", ident_str);
    let update_string = format!("UPDATE {} SET {} WHERE id = ${}", ident_str, update_fields, insert_index);

    let output = quote! {
        #[crash_orm::async_trait::async_trait]
        impl crash_orm::Entity for #ident {
            type Output = #ident;

            fn load_from_row(row: &crash_orm::tokio_postgres::Row) -> Self::Output {
                #ident {
                    #select_fields
                }
            }

            async fn get_by_id(connection: &crash_orm::DatabaseConnection, id: u32) -> crash_orm::Result<Self::Output> {
                let row = connection.query_one(#select_by_id_string, &[&id]).await?;
                Ok(Self::load_from_row(&row))
            }

            async fn get_all(connection: &crash_orm::DatabaseConnection) -> crash_orm::Result<Vec<Self::Output>> {
                let rows = connection.query(#select_all_string, &[]).await?;
                Ok(rows.iter().map(|v| Self::load_from_row(v)).collect::<Vec<Self>>())
            }

            async fn insert_get_id(&self, connection: &crash_orm::DatabaseConnection) -> crash_orm::Result<u32> {
                let row = connection.query(#insert_string,&[#insert_field_self_values]).await?;
                Ok(row.get(0).unwrap().get(0))
            }

            async fn insert_set_id(&mut self, connection: &crash_orm::DatabaseConnection) -> crash_orm::Result<()> {
                let id = self.insert_get_id(connection).await?;
                self.id = Some(id);
                Ok(())
            }

            async fn remove(&mut self, connection: &crash_orm::DatabaseConnection) -> crash_orm::Result<()> {
                if self.id.is_none() {
                    return Ok(());
                }

                connection.execute(#delete_string, &[&self.id]).await?;
                self.id = None;
                Ok(())
            }

            async fn update(&self, connection: &crash_orm::DatabaseConnection) -> crash_orm::Result<()> {
                if self.id.is_none() {
                    return Err(crash_orm::Error::from_str("You can't update an entity without an id."));
                }

                connection.execute(#update_string,&[#insert_field_self_values &self.id],).await?;

                Ok(())
            }

            async fn persist(&mut self, connection: &crash_orm::DatabaseConnection) -> crash_orm::Result<()> {
                if self.id.is_none() {
                    self.insert_set_id(connection).await
                } else {
                    self.update(connection).await
                }
            }
        }
    };

    output.into()
}