use crate::util::{ident_to_table_name, rust_to_postgres_type};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

pub fn derive_schema_impl(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    let Data::Struct(struct_data) = derive_input.data else {
        panic!("only structs are supported")
    };

    let mut create_fields_string = String::new();

    let ident = derive_input.ident;
    let ident_str = ident_to_table_name(&ident);

    for field in struct_data.fields {
        let field_name = field.ident.clone().unwrap().to_string();
        let column_type = rust_to_postgres_type(&field.ty, &*field_name);

        if column_type.is_none() {
            continue;
        }

        create_fields_string.push_str(&*format!("{} {}", field_name, column_type.unwrap()));

        if &*field_name == "id" {
            create_fields_string.push_str(&*format!(
                " DEFAULT nextval('{}_id_seq'::regclass)",
                ident_str
            ));
        }

        create_fields_string.push_str(",");
    }

    create_fields_string.push_str("PRIMARY KEY (id)");

    let create_string = format!(
        "CREATE TABLE public.{}({});",
        ident_str, create_fields_string
    );

    let sequence_create = format!("CREATE SEQUENCE {}_id_seq", ident_str);
    let sequence_created_alter = format!("ALTER SEQUENCE {0}_id_seq OWNED BY {0}.id", ident_str);

    let drop_string = format!("DROP TABLE IF EXISTS {} CASCADE", ident_str);
    let truncate_string = format!("TRUNCATE {} RESTART IDENTITY CASCADE", ident_str);
    let table_exists_string = format!(
        "SELECT EXISTS(SELECT FROM pg_tables WHERE schemaname = 'public' AND tablename = '{}')",
        ident_str
    );

    let output = quote! {
        #[crash_orm::async_trait::async_trait]
        impl crash_orm::Schema for #ident {
            async fn create_table(connection: &impl crash_orm::DatabaseConnection) -> crash_orm::Result<()> {
                connection.execute_query(#sequence_create, &[]).await?;
                connection.execute_query(#create_string, &[]).await?;
                connection.execute_query(#sequence_created_alter, &[]).await?;

                Ok(())
            }

            async fn drop_table(connection: &impl crash_orm::DatabaseConnection) -> crash_orm::Result<()> {
                connection.execute_query(#drop_string, &[]).await?;

                Ok(())
            }

            async fn truncate_table(connection: &impl crash_orm::DatabaseConnection) -> crash_orm::Result<()> {
                connection.execute_query(#truncate_string, &[]).await?;

                Ok(())
            }

            async fn table_exists(connection: &impl crash_orm::DatabaseConnection) -> crash_orm::Result<bool> {
                let row = connection.query_single(#table_exists_string, &[]).await?;
                let exists: bool = row.get(0);
                Ok(exists)
            }
        }
    };

    output.into()
}
