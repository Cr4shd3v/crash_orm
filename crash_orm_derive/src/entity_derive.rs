use proc_macro::TokenStream;

use quote::{quote, ToTokens};
use syn::{Attribute, Data, DeriveInput, Field, Ident, parse_macro_input};

use crate::util::{
    extract_generic_type, extract_generic_type_ignore_option, get_type_string, ident_to_table_name,
    is_relation, is_relation_value_holder, string_to_table_name,
};

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
    let mut functions = quote!();

    let mut all_index = 0usize;
    let mut insert_index = 0usize;
    let mut update_index = 0usize;

    let mut primary_type = None;

    for field in &struct_data.fields {
        let field_ident = field.ident.as_ref().unwrap();
        let field_ident_str = field_ident.to_string();
        if field_ident_str == "id" {
            primary_type = Some(field.ty.clone());
        }
    }

    let Some(primary_type) = primary_type else {
        panic!("The entity {} has no primary key", ident_str);
    };

    let Some(primary_type) = extract_generic_type(&primary_type, 1) else {
        panic!("The identifier for entity {} must be an option", ident_str);
    };

    let primary_type_str = get_type_string(&primary_type);

    for field in struct_data.fields {
        let field_ident = field.ident.as_ref().unwrap();
        let field_ident_str = field_ident.to_string();
        let field_ident_upper = Ident::new(&*field_ident_str.to_uppercase(), field_ident.span());
        let field_type = &field.ty;

        if is_relation(field_type) {
            let field_type_name = get_type_string(field_type);
            let (field_type_name, is_option) = if field_type_name == "Option" {
                (
                    get_type_string(&extract_generic_type(field_type, 1).unwrap()),
                    true,
                )
            } else {
                (field_type_name, false)
            };
            let entity_type = extract_generic_type_ignore_option(field_type, 1).unwrap();
            let entity_table_name = string_to_table_name(get_type_string(&entity_type));
            let set_function_ident = Ident::new(&*format!("set_{}", field_ident_str), ident.span());
            let get_function_ident = Ident::new(&*format!("get_{}", field_ident_str), ident.span());

            if field_type_name == "OneToMany" {
                let mapped_by = get_mapped_by_attribute(&field);

                if mapped_by.is_none() {
                    panic!("The attribute \"mapped_by\" is required on OneToMany objects");
                }

                select_fields.extend(quote! {
                    #field_ident: crash_orm::OneToMany::new(),
                });

                let mapped_by = parse_mapped_by_arg(mapped_by.unwrap());

                let query = format!(
                    "SELECT * FROM {} WHERE {} = $1",
                    entity_table_name, mapped_by
                );

                functions.extend(quote! {
                    async fn #get_function_ident(&self, connection: &impl crash_orm::DatabaseConnection) -> crash_orm::Result<Vec<#entity_type>> {
                        let rows = connection.query_many(#query, &[&self.id]).await?;
                        Ok(rows.iter().map(|v| #entity_type::load_from_row(v)).collect::<Vec<#entity_type>>())
                    }
                });

                continue;
            } else if field_type_name == "ManyToOne" {
                if is_option {
                    functions.extend(quote! {
                        fn #set_function_ident(&mut self, #field_ident: Option<&impl Entity<#entity_type, #primary_type>>) -> crash_orm::Result<()> {
                            self.#field_ident = if #field_ident.is_some() {
                                Some(crash_orm::ManyToOne::from(#field_ident.unwrap())?)
                            } else {
                                None
                            };

                            Ok(())
                        }

                        async fn #get_function_ident(&self, connection: &impl crash_orm::DatabaseConnection) -> crash_orm::Result<Option<#entity_type>> {
                            if self.#field_ident.is_some() {
                                Ok(Some(#entity_type::get_by_id(connection, self.#field_ident.as_ref().unwrap().target_id).await?))
                            } else {
                                Ok(None)
                            }
                        }
                    });
                } else {
                    functions.extend(quote! {
                        fn #set_function_ident(&mut self, #field_ident: &impl Entity<#entity_type, #primary_type>) -> crash_orm::Result<()> {
                            self.#field_ident = crash_orm::ManyToOne::from(#field_ident)?;

                            Ok(())
                        }

                        async fn #get_function_ident(&self, connection: &impl crash_orm::DatabaseConnection) -> crash_orm::Result<#entity_type> {
                            #entity_type::get_by_id(connection, self.#field_ident.target_id).await
                        }
                    });
                }
            } else if field_type_name == "OneToOne" {
                if is_option {
                    functions.extend(quote! {
                        fn #set_function_ident(&mut self, #field_ident: Option<&impl Entity<#entity_type, #primary_type>>) -> crash_orm::Result<()> {
                            self.#field_ident = if #field_ident.is_some() {
                                Some(crash_orm::OneToOne::from(#field_ident.unwrap())?)
                            } else {
                                None
                            };

                            Ok(())
                        }

                        async fn #get_function_ident(&self, connection: &impl crash_orm::DatabaseConnection) -> crash_orm::Result<Option<#entity_type>> {
                            if self.#field_ident.is_some() {
                                Ok(Some(#entity_type::get_by_id(connection, self.#field_ident.as_ref().unwrap().target_id).await?))
                            } else {
                                Ok(None)
                            }
                        }
                    });
                } else {
                    functions.extend(quote! {
                        fn #set_function_ident(&mut self, #field_ident: &impl Entity<#entity_type, #primary_type>) -> crash_orm::Result<()> {
                            self.#field_ident = crash_orm::OneToOne::from(#field_ident)?;

                            Ok(())
                        }

                        async fn #get_function_ident(&self, connection: &impl crash_orm::DatabaseConnection) -> crash_orm::Result<#entity_type> {
                            #entity_type::get_by_id(connection, self.#field_ident.target_id).await
                        }
                    });
                }
            } else if field_type_name == "OneToOneRef" {
                let mapped_by = get_mapped_by_attribute(&field);

                if mapped_by.is_none() {
                    panic!("The attribute \"mapped_by\" is required on OneToOneRef objects");
                }

                select_fields.extend(quote! {
                    #field_ident: crash_orm::OneToOneRef::new(),
                });

                let mapped_by = parse_mapped_by_arg(mapped_by.unwrap());
                let query = format!(
                    "SELECT * FROM {} WHERE {} = $1",
                    entity_table_name, mapped_by
                );

                functions.extend(quote! {
                    async fn #get_function_ident(&self, connection: &impl crash_orm::DatabaseConnection) -> crash_orm::Result<#entity_type> {
                        let row = connection.query_single(#query, &[&self.id]).await?;
                        Ok(#entity_type::load_from_row(&row))
                    }
                });

                continue;
            }
        }

        select_fields.extend(quote! {
            #field_ident: row.get(#all_index),
        });

        all_index += 1;

        if field_ident_str != "id" {
            column_consts.extend(quote! {
                pub const #field_ident_upper: crash_orm::EntityColumn::<#field_type, #ident, #primary_type> = crash_orm::EntityColumn::new(#field_ident_str);
            });

            if is_relation_value_holder(&field_type) {
                let field_ident_upper_id = Ident::new(
                    &*format!("{}_ID", field_ident_str.to_uppercase()),
                    field_ident.span(),
                );
                let Some(target_entity_id_type) = extract_generic_type_ignore_option(field_type, 2) else {
                    panic!("Missing generic parameter at {}", field_ident_str);
                };
                column_consts.extend(quote! {
                    pub const #field_ident_upper_id: crash_orm::EntityColumn::<#target_entity_id_type, #ident, #primary_type> = crash_orm::EntityColumn::new(#field_ident_str);
                });
            }

            insert_field_names.extend(quote! {
                #field_ident_str,
            });

            insert_field_self_values.extend(quote! {
                &self.#field_ident,
            });

            update_index += 1;
            update_fields.push_str(&*format!("{} = ${}", field_ident_str, update_index));

            insert_index += 1;
            insert_field_self_values_format.push_str(&*format!("${},", insert_index));
        } else if primary_type_str == "Uuid" {
            insert_field_names.extend(quote! {
                #field_ident_str,
            });

            insert_field_self_values.extend(quote! {
                &self.#field_ident,
            });

            insert_index += 1;

            insert_field_self_values_format.push_str(&*format!("${},", insert_index));
        }

        all_field_self_values_format.push_str(&*format!("${},", all_index));
    }

    insert_index += 1;

    let insert_field_names = insert_field_names.to_string();
    let insert_field_names = insert_field_names.strip_suffix(",").unwrap();
    let insert_field_self_values_format =
        insert_field_self_values_format.strip_suffix(",").unwrap();

    let select_by_id_string = format!("SELECT * FROM {} WHERE id = $1", ident_str);
    let select_all_string = format!("SELECT * FROM {}", ident_str);
    let count_string = format!("SELECT COUNT(*) FROM {}", ident_str);
    let insert_string = format!(
        "INSERT INTO {}({}) VALUES ({}) RETURNING id",
        ident_str, insert_field_names, insert_field_self_values_format
    );
    let delete_string = format!("DELETE FROM {} WHERE id = $1", ident_str);
    let update_string = format!(
        "UPDATE {} SET {} WHERE id = ${}",
        ident_str, update_fields, insert_index
    );
    let ident_column = Ident::new(&*format!("{}Column", ident.to_string()), ident.span());

    let mut output = quote! {
        #vis struct #ident_column;

        impl #ident_column {
            #column_consts
        }

        impl crash_orm::BaseColumn<#ident, #primary_type> for #ident_column {}

        #[crash_orm::async_trait::async_trait]
        impl crash_orm::Entity<#ident, #primary_type> for #ident {
            const TABLE_NAME: &'static str = #ident_str;

            type ColumnType = #ident_column;

            fn get_id(&self) -> Option<#primary_type> {
                self.id
            }

            fn load_from_row(row: &crash_orm::postgres::Row) -> #ident {
                #ident {
                    #select_fields
                }
            }

            async fn get_by_id(connection: &impl crash_orm::DatabaseConnection, id: #primary_type) -> crash_orm::Result<#ident> {
                let row = connection.query_single(#select_by_id_string, &[&id]).await?;
                Ok(Self::load_from_row(&row))
            }

            async fn get_all(connection: &impl crash_orm::DatabaseConnection) -> crash_orm::Result<Vec<#ident>> {
                let rows = connection.query_many(#select_all_string, &[]).await?;
                Ok(rows.iter().map(|v| Self::load_from_row(v)).collect::<Vec<Self>>())
            }

            async fn count(connection: &impl crash_orm::DatabaseConnection) -> crash_orm::Result<i64> {
                let row = connection.query_single(#count_string, &[]).await?;
                Ok(row.get(0))
            }

            async fn insert_get_id(&self, connection: &impl crash_orm::DatabaseConnection) -> crash_orm::Result<#primary_type> {
                let rows = connection.query_many(#insert_string,&[#insert_field_self_values]).await?;
                Ok(rows.get(0).unwrap().get(0))
            }

            async fn insert_set_id(&mut self, connection: &impl crash_orm::DatabaseConnection) -> crash_orm::Result<()> {
                let id = self.insert_get_id(connection).await?;
                self.id = Some(id);
                Ok(())
            }

            async fn remove(&mut self, connection: &impl crash_orm::DatabaseConnection) -> crash_orm::Result<()> {
                if self.id.is_none() {
                    return Ok(());
                }

                connection.execute_query(#delete_string, &[&self.id]).await?;
                self.id = None;
                Ok(())
            }

            async fn update(&self, connection: &impl crash_orm::DatabaseConnection) -> crash_orm::Result<()> {
                if self.id.is_none() {
                    return Err(crash_orm::Error::from_str("You can't update an entity without an id."));
                }

                connection.execute_query(#update_string,&[#insert_field_self_values &self.id]).await?;

                Ok(())
            }

            async fn persist(&mut self, connection: &impl crash_orm::DatabaseConnection) -> crash_orm::Result<()> {
                if self.id.is_none() {
                    self.insert_set_id(connection).await
                } else {
                    self.update(connection).await
                }
            }
        }
    };

    if !functions.is_empty() {
        output.extend(quote! {
            impl #ident {
                #functions
            }
        });
    }

    output.into()
}

fn get_mapped_by_attribute(field: &Field) -> Option<&Attribute> {
    field.attrs.iter().find(|a| {
        a.meta
            .path()
            .segments
            .last()
            .unwrap()
            .to_token_stream()
            .to_string()
            == "mapped_by"
    })
}

fn parse_mapped_by_arg(attribute: &Attribute) -> String {
    let mapped_by = attribute.parse_args::<syn::LitStr>();

    if mapped_by.is_err() {
        panic!("The attribute \"mapped_by\" requires a string as the argument");
    }

    mapped_by.unwrap().value()
}
