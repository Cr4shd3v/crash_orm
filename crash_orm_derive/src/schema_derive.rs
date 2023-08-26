use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{Data, DeriveInput, parse_macro_input};

pub fn derive_schema_impl(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    let Data::Struct(struct_data) = derive_input.data else {
        panic!("only structs are supported")
    };

    let mut create_fields_string = String::new();

    let ident = derive_input.ident;
    let ident_str = ident.to_string();

    for field in struct_data.fields {
        let syn::Type::Path(path) = field.ty else { panic!("unsupported") };
        let path = path.into_token_stream().to_string().replace(" ", "");
        let field_name = field.ident.unwrap().to_string();

        let (path, _nullable) = if path.starts_with("Option<") {
            (path.strip_prefix("Option<").unwrap().strip_suffix(">").unwrap().to_string(), &*field_name != "id")
        } else {
            (path, false)
        };

        let column_type = match &*path {
            "u32" => "oid",
            "String" => "text",
            _ => panic!("unsupported type {}", path),
        };

        create_fields_string.push_str(&*format!("{} {} NOT NULL", field_name, column_type));

        if &*field_name == "id" {
            create_fields_string.push_str(&*format!(" DEFAULT nextval('{}_id_seq'::regclass)", ident_str));
        }

        create_fields_string.push_str(",");
    }

    create_fields_string.push_str("PRIMARY KEY (id)");

    let create_string = format!("CREATE TABLE public.{}({});", ident_str, create_fields_string);

    let sequence_create = format!("CREATE SEQUENCE {}_id_seq", ident_str);
    let sequence_created_alter = format!("ALTER SEQUENCE {0}_id_seq OWNED BY {0}.id", ident_str);

    let drop_string = format!("DROP TABLE IF EXISTS {} CASCADE", ident_str);
    let truncate_string = format!("TRUNCATE {} RESTART IDENTITY", ident_str);

    let output = quote! {
        #[crash_orm::async_trait::async_trait]
        impl crash_orm::Schema for #ident {
            async fn create_table(connection: &crash_orm::DatabaseConnection) -> crash_orm::Result<()> {
                connection.execute(#sequence_create, &[]).await?;
                connection.execute(#create_string, &[]).await?;
                connection.execute(#sequence_created_alter, &[]).await?;

                Ok(())
            }

            async fn drop_table(connection: &crash_orm::DatabaseConnection) -> crash_orm::Result<()> {
                connection.execute(#drop_string, &[]).await?;

                Ok(())
            }

            async fn truncate_table(connection: &crash_orm::DatabaseConnection) -> crash_orm::Result<()> {
                connection.execute(#truncate_string, &[]).await?;

                Ok(())
            }
        }
    };

    output.into()
}