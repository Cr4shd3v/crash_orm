use proc_macro::TokenStream;

use quote::quote;
use syn::__private::Span;
use syn::{parse_macro_input, Attribute, Data, DeriveInput, Ident};
use crate::reserved_keywords::escape_reserved_keywords;
use crate::util::{extract_generic_type, extract_generic_type_ignore_option, get_attribute_by_name, get_type_string, ident_to_table_name, is_relation, is_relation_value_holder, string_to_table_name};

pub fn derive_entity_impl(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    let Data::Struct(struct_data) = derive_input.data else {
        panic!("only structs are supported")
    };

    let ident = derive_input.ident;
    let ident_str = ident_to_table_name(&ident);
    let vis = derive_input.vis;
    
    let mut all_field_self_values_format = String::new();
    let mut insert_field_names = vec![];
    let mut insert_field_self_values = quote!();
    let mut insert_field_values = quote!();
    let mut update_field_self_values = quote!();
    let mut insert_field_self_values_format = String::new();
    let mut update_fields = vec![];
    let mut column_consts = quote!();
    let mut functions = quote!();
    let mut create_fields = quote!();
    let mut create_fields_mapping = quote!();

    let mut all_index = 0usize;
    let mut insert_index = 0usize;
    let mut update_index = 0usize;

    let (primary_type, primary_field_name) = {
        let mut primary_type_id_field = None;
        let mut defined_primary_key = None;
        let mut defined_primary_key_name = None;

        for field in &struct_data.fields {
            let field_ident = field.ident.as_ref().unwrap();
            let field_ident_str = field_ident.to_string();
            if field_ident_str == "id" {
                primary_type_id_field = Some(field.ty.clone());
            }

            let primary_key = get_attribute_by_name(field, "primary_key").is_some();
            if primary_key {
                defined_primary_key = Some(field.ty.clone());
                defined_primary_key_name = Some(field_ident_str);
            }
        }

        if defined_primary_key.is_some() {
            (defined_primary_key.unwrap(), defined_primary_key_name.unwrap())
        } else {
            (primary_type_id_field.expect(&*format!("The entity {} has no primary key", ident_str)), "id".to_string())
        }
    };
    let primary_key_ident = Ident::new(&*primary_field_name, Span::call_site());

    let primary_type_str = get_type_string(&primary_type);

    if primary_type_str == "Option" {
        panic!("The primary key must not be an Option!");
    }

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
                let mapped_by = get_attribute_by_name(&field, "mapped_by");

                if mapped_by.is_none() {
                    panic!("The attribute \"mapped_by\" is required on OneToMany objects");
                }

                let mapped_by = parse_mapped_by_arg(mapped_by.unwrap());

                let query = format!(
                    "SELECT * FROM {} WHERE {} = $1",
                    entity_table_name, mapped_by
                );

                functions.extend(quote! {
                    async fn #get_function_ident(&self, connection: &impl crash_orm::prelude::DatabaseConnection) -> crash_orm::Result<Vec<#entity_type>> {
                        let rows = connection.query_many(#query, &[&self.#primary_key_ident]).await?;
                        use crash_orm::prelude::{Entity, ResultMapping};
                        Ok(rows.into_iter().map(|v| #entity_type::from_row(v)).collect::<Vec<#entity_type>>())
                    }
                });

                create_fields_mapping.extend(quote! {
                    #field_ident: crash_orm::prelude::OneToMany::new(),
                });

                continue;
            } else if field_type_name == "ManyToOne" {
                if is_option {
                    functions.extend(quote! {
                        fn #set_function_ident(&mut self, #field_ident: Option<&#entity_type>) -> crash_orm::Result<()> {
                            self.#field_ident = if #field_ident.is_some() {
                                Some(crash_orm::prelude::ManyToOne::from(#field_ident.unwrap())?)
                            } else {
                                None
                            };

                            Ok(())
                        }

                        async fn #get_function_ident(&self, connection: &impl crash_orm::prelude::DatabaseConnection) -> crash_orm::Result<Option<#entity_type>> {
                            if self.#field_ident.is_some() {
                                use crash_orm::entity::PrimaryKeyEntity;
                                Ok(Some(#entity_type::get_by_primary(connection, self.#field_ident.as_ref().unwrap().target_id).await?))
                            } else {
                                Ok(None)
                            }
                        }
                    });
                } else {
                    functions.extend(quote! {
                        fn #set_function_ident(&mut self, #field_ident: &#entity_type) -> crash_orm::Result<()> {
                            self.#field_ident = crash_orm::prelude::ManyToOne::from(#field_ident)?;

                            Ok(())
                        }

                        async fn #get_function_ident(&self, connection: &impl crash_orm::prelude::DatabaseConnection) -> crash_orm::Result<#entity_type> {
                            use crash_orm::entity::PrimaryKeyEntity;
                            #entity_type::get_by_primary(connection, self.#field_ident.target_id).await
                        }
                    });
                }
            } else if field_type_name == "OneToOne" {
                if is_option {
                    functions.extend(quote! {
                        fn #set_function_ident(&mut self, #field_ident: Option<&#entity_type>) -> crash_orm::Result<()> {
                            self.#field_ident = if #field_ident.is_some() {
                                Some(crash_orm::prelude::OneToOne::from(#field_ident.unwrap())?)
                            } else {
                                None
                            };

                            Ok(())
                        }

                        async fn #get_function_ident(&self, connection: &impl crash_orm::prelude::DatabaseConnection) -> crash_orm::Result<Option<#entity_type>> {
                            if self.#field_ident.is_some() {
                                use crash_orm::prelude::PrimaryKeyEntity;
                                Ok(Some(#entity_type::get_by_primary(connection, self.#field_ident.as_ref().unwrap().target_id).await?))
                            } else {
                                Ok(None)
                            }
                        }
                    });
                } else {
                    functions.extend(quote! {
                        fn #set_function_ident(&mut self, #field_ident: &#entity_type) -> crash_orm::Result<()> {
                            self.#field_ident = crash_orm::prelude::OneToOne::from(#field_ident)?;

                            Ok(())
                        }

                        async fn #get_function_ident(&self, connection: &impl crash_orm::prelude::DatabaseConnection) -> crash_orm::Result<#entity_type> {
                            use crash_orm::entity::PrimaryKeyEntity;
                            #entity_type::get_by_primary(connection, self.#field_ident.target_id).await
                        }
                    });
                }
            } else if field_type_name == "OneToOneRef" {
                let mapped_by = get_attribute_by_name(&field, "mapped_by");

                if mapped_by.is_none() {
                    panic!("The attribute \"mapped_by\" is required on OneToOneRef objects");
                }

                let mapped_by = parse_mapped_by_arg(mapped_by.unwrap());
                let query = format!(
                    "SELECT * FROM {} WHERE {} = $1",
                    entity_table_name, mapped_by
                );

                functions.extend(quote! {
                    async fn #get_function_ident(&self, connection: &impl crash_orm::prelude::DatabaseConnection) -> crash_orm::Result<#entity_type> {
                        use crash_orm::prelude::{Entity, ResultMapping};
                        let row = connection.query_single(#query, &[&self.#primary_key_ident]).await?;
                        Ok(#entity_type::from_row(row))
                    }
                });

                create_fields_mapping.extend(quote! {
                    #field_ident: crash_orm::prelude::OneToOneRef::new(),
                });

                continue;
            }
        }

        all_index += 1;

        column_consts.extend(quote! {
            #[allow(missing_docs)]
            pub const #field_ident_upper: crash_orm::prelude::EntityColumn::<#field_type, #ident> = crash_orm::prelude::EntityColumn::new(#field_ident_str);
        });

        if field_ident_str != primary_field_name {
            if is_relation_value_holder(&field_type) {
                let field_ident_upper_id = Ident::new(
                    &*format!("{}_PRIMARY", field_ident_str.to_uppercase()),
                    field_ident.span(),
                );
                let Some(target_entity_id_type) = extract_generic_type_ignore_option(field_type, 2) else {
                    panic!("Missing generic parameter at {}", field_ident_str);
                };
                column_consts.extend(quote! {
                    pub const #field_ident_upper_id: crash_orm::prelude::EntityColumn::<#target_entity_id_type, #ident> = crash_orm::prelude::EntityColumn::new(#field_ident_str);
                });
            }

            insert_field_self_values.extend(quote! {
                &self.#field_ident,
            });

            insert_field_values.extend(quote! {
                &self.#field_ident,
            });

            update_field_self_values.extend(quote! {
                &self.#field_ident,
            });

            update_index += 1;
            update_fields.push(format!("{} = ${}", escape_reserved_keywords(&field_ident_str), update_index));

            insert_index += 1;
            insert_field_self_values_format.push_str(&*format!("${},", insert_index));
            insert_field_names.push(escape_reserved_keywords(&field_ident_str));

            create_fields.extend(quote! {
                pub #field_ident: #field_type,
            });

            create_fields_mapping.extend(quote! {
                #field_ident: self.#field_ident,
            });
        } else if primary_type_str == "Uuid" {
            insert_field_names.push(escape_reserved_keywords(&field_ident_str));

            insert_field_self_values.extend(quote! {
                &self.#field_ident,
            });

            // If no uuid generation is configured, the user must provide it in the Create struct
            #[cfg(not(any(feature = "uuid-gen-v4", feature = "uuid-gen-v7")))]
            create_fields.extend(quote! {
                pub #field_ident: #field_type,
            });
            #[cfg(not(any(feature = "uuid-gen-v4", feature = "uuid-gen-v7")))]
            create_fields_mapping.extend(quote! {
                #field_ident: self.#field_ident,
            });

            #[cfg(feature = "uuid-gen-v4")]
            create_fields_mapping.extend(quote! {
                #field_ident: uuid::Uuid::new_v4(),
            });

            #[cfg(feature = "uuid-gen-v7")]
            create_fields_mapping.extend(quote! {
                #field_ident: uuid::Uuid::now_v7(),
            });

            #[cfg(not(any(feature = "uuid-gen-v4", feature = "uuid-gen-v7")))]
            insert_field_values.extend(quote! {
                &self.#field_ident,
            });
            #[cfg(feature = "uuid-gen-v4")]
            insert_field_values.extend(quote! {
                &uuid::Uuid::new_v4(),
            });
            #[cfg(feature = "uuid-gen-v7")]
            insert_field_values.extend(quote! {
                &uuid::Uuid::now_v7(),
            });

            insert_index += 1;

            insert_field_self_values_format.push_str(&*format!("${},", insert_index));
        } else {
            create_fields_mapping.extend(quote! {
                #field_ident: Default::default(),
            });
        }

        all_field_self_values_format.push_str(&*format!("${},", all_index));
    }

    insert_index += 1;

    let insert_field_names = insert_field_names.join(",");
    let insert_field_self_values_format =
        insert_field_self_values_format.strip_suffix(",").unwrap_or("");

    let select_by_id_string = format!("SELECT * FROM {} WHERE {} = $1", ident_str, primary_field_name);
    let select_all_string = format!("SELECT * FROM {}", ident_str);
    let count_string = format!("SELECT COUNT({}) FROM {}", primary_field_name, ident_str);
    let insert_string = if insert_field_names.is_empty() {
        format!("INSERT INTO {} DEFAULT VALUES RETURNING {}", ident_str, primary_field_name)
    } else {
        format!(
            "INSERT INTO {}({}) VALUES ({}) RETURNING {}",
            ident_str, insert_field_names, insert_field_self_values_format, primary_field_name
        )
    };
    let delete_string = format!("DELETE FROM {} WHERE {} = $1", ident_str, primary_field_name);

    let update_statement = if update_fields.is_empty() {
        quote!()
    } else {
        let update_string = format!(
            "UPDATE {} SET {} WHERE {} = ${}",
            ident_str, update_fields.join(","), primary_field_name, insert_index
        );
        quote! {
            connection.execute_query(#update_string,&[#update_field_self_values &self.#primary_key_ident]).await?;
        }
    };

    let ident_column = Ident::new(&*format!("{}Column", ident.to_string()), ident.span());
    let ident_column_doc = format!("Column struct for [{}]", ident_str);
    let ident_create = Ident::new(&*format!("{}Create", ident), ident.span());
    let create_doc_text = format!("Creation struct for {}", ident_str);

    #[cfg(not(feature = "serialize"))]
    let create_macro = quote!();
    #[cfg(feature = "serialize")]
    let create_macro = quote! {
        #[derive(serde::Serialize, serde::Deserialize)]
    };

    let mut output = quote! {
        #[doc=#ident_column_doc]
        #vis struct #ident_column;

        impl #ident_column {
            #column_consts
        }

        #create_macro
        #[doc=#create_doc_text]
        #[allow(missing_docs)]
        #vis struct #ident_create {
            #create_fields
        }

        #[crash_orm::async_trait::async_trait]
        impl crash_orm::prelude::CreateEntity<#ident> for #ident_create {
            fn into_entity(self) -> #ident {
                #ident {
                    #create_fields_mapping
                }
            }
        }

        #[crash_orm::async_trait::async_trait]
        impl crash_orm::prelude::Entity for #ident {
            const TABLE_NAME: &'static str = #ident_str;

            const __INSERT_FIELD_NAMES: &'static str = #insert_field_names;

            type ColumnType = #ident_column;

            fn get_values(&self) -> Vec<&(dyn crash_orm::postgres::types::ToSql + Sync)> {
                vec![
                    #insert_field_self_values
                ]
            }

            async fn get_all(connection: &impl crash_orm::prelude::DatabaseConnection) -> crash_orm::Result<Vec<#ident>> {
                let rows = connection.query_many(#select_all_string, &[]).await?;
                use crash_orm::prelude::ResultMapping;
                Ok(rows.into_iter().map(|v| Self::from_row(v)).collect::<Vec<Self>>())
            }

            async fn count(connection: &impl crash_orm::prelude::DatabaseConnection) -> crash_orm::Result<i64> {
                let row = connection.query_single(#count_string, &[]).await?;
                Ok(row.get(0))
            }

            async fn insert(&mut self, connection: &impl crash_orm::prelude::DatabaseConnection) -> crash_orm::Result<()> {
                let row = connection.query_single(#insert_string,&[#insert_field_values]).await?;
                self.#primary_key_ident = row.get(0);
                Ok(())
            }

            async fn remove(&mut self, connection: &impl crash_orm::prelude::DatabaseConnection) -> crash_orm::Result<()> {
                connection.execute_query(#delete_string, &[&self.#primary_key_ident]).await?;
                Ok(())
            }

            async fn update(&self, connection: &impl crash_orm::prelude::DatabaseConnection) -> crash_orm::Result<()> {
                #update_statement

                Ok(())
            }
        }

        #[crash_orm::async_trait::async_trait]
        impl crash_orm::prelude::PrimaryKeyEntity<#primary_type> for #ident {
            fn get_primary(&self) -> #primary_type {
                self.#primary_key_ident
            }

            async fn get_by_primary(connection: &impl crash_orm::prelude::DatabaseConnection, #primary_key_ident: #primary_type) -> crash_orm::Result<#ident> {
                let row = connection.query_single(#select_by_id_string, &[&#primary_key_ident]).await?;
                use crash_orm::prelude::{Entity, ResultMapping};
                Ok(Self::from_row(row))
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

fn parse_mapped_by_arg(attribute: &Attribute) -> String {
    let mapped_by = attribute.parse_args::<syn::LitStr>();

    if mapped_by.is_err() {
        panic!("The attribute \"mapped_by\" requires a string as the argument");
    }

    mapped_by.unwrap().value()
}
