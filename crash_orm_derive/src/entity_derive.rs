use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Ident, parse_macro_input};
use crate::util::{ident_to_table_name, is_relation};

pub fn derive_entity_impl(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    let Data::Struct(struct_data) = derive_input.data else {
        panic!("only structs are supported")
    };

    let ident = derive_input.ident;
    let ident_str = ident_to_table_name(&ident);
    let vis = derive_input.vis;

    let mut select_fields = quote!();
    let mut all_field_self_values_format = String::new();
    let mut insert_field_names = quote!();
    let mut insert_field_self_values = quote!();
    let mut insert_field_self_values_format = String::new();
    let mut update_fields = String::new();
    let mut column_consts = quote!();

    let mut all_index = 0usize;
    let mut insert_index = 0usize;
    for field in struct_data.fields {
        let field_ident = field.ident.unwrap();
        let field_ident_str = field_ident.to_string();
        let field_ident_upper = Ident::new(&*field_ident_str.to_uppercase(), field_ident.span());

        select_fields.extend(quote! {
            #field_ident: row.get(#all_index),
        });

        if field_ident.to_string() != "id" {
            let field_type = field.ty;

            column_consts.extend(quote! {
                pub const #field_ident_upper: crash_orm::EntityColumn::<#field_type, #ident> = crash_orm::EntityColumn::<#field_type, #ident>::new(#field_ident_str);
            });

            if is_relation(&field_type) {
                let field_ident_upper_id = Ident::new(&*format!("{}_ID", field_ident_str.to_uppercase()), field_ident.span());
                column_consts.extend(quote! {
                    pub const #field_ident_upper_id: crash_orm::EntityColumn::<u32, #ident> = crash_orm::EntityColumn::<u32, #ident>::new(#field_ident_str);
                });
            }

            insert_field_names.extend(quote! {
                #field_ident_str,
            });

            insert_field_self_values.extend(quote! {
                &self.#field_ident,
            });

            insert_index += 1;

            update_fields.push_str(&*format!("{} = ${}", field_ident_str, insert_index));
            insert_field_self_values_format.push_str(&*format!("${},", insert_index));
        } else {
            column_consts.extend(quote! {
                pub const #field_ident_upper: crash_orm::EntityColumn::<u32, #ident> = crash_orm::EntityColumn::<u32, #ident>::new(#field_ident_str);
            });
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
    let count_string = format!("SELECT COUNT(*) FROM {}", ident_str);
    let insert_string = format!("INSERT INTO {}({}) VALUES ({}) RETURNING id", ident_str, insert_field_names, insert_field_self_values_format);
    let delete_string = format!("DELETE FROM {} WHERE id = $1", ident_str);
    let update_string = format!("UPDATE {} SET {} WHERE id = ${}", ident_str, update_fields, insert_index);
    let ident_column = Ident::new(&*format!("{}Column", ident.to_string()), ident.span());

    let output = quote! {
        #vis struct #ident_column;

        impl #ident_column {
            #column_consts
        }

        #[crash_orm::async_trait::async_trait]
        impl crash_orm::Entity<#ident> for #ident {
            const TABLE_NAME: &'static str = #ident_str;

            fn get_id(&self) -> Option<u32> {
                self.id
            }

            fn load_from_row(row: &crash_orm::tokio_postgres::Row) -> #ident {
                #ident {
                    #select_fields
                }
            }

            async fn get_by_id(connection: &crash_orm::DatabaseConnection, id: u32) -> crash_orm::Result<#ident> {
                let row = connection.query_one(#select_by_id_string, &[&id]).await?;
                Ok(Self::load_from_row(&row))
            }

            async fn get_all(connection: &crash_orm::DatabaseConnection) -> crash_orm::Result<Vec<#ident>> {
                let rows = connection.query(#select_all_string, &[]).await?;
                Ok(rows.iter().map(|v| Self::load_from_row(v)).collect::<Vec<Self>>())
            }

            async fn count(connection: &crash_orm::DatabaseConnection) -> crash_orm::Result<i64> {
                let row = connection.query_one(#count_string, &[]).await?;
                Ok(row.get(0))
            }

            async fn insert_get_id(&self, connection: &crash_orm::DatabaseConnection) -> crash_orm::Result<u32> {
                let rows = connection.query(#insert_string,&[#insert_field_self_values]).await?;
                Ok(rows.get(0).unwrap().get(0))
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