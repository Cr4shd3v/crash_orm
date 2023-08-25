use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{Data, DeriveInput, parse_macro_input};

pub fn derive_schema_impl(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    let Data::Struct(struct_data) = derive_input.data else {
        panic!("only structs are supported")
    };

    let mut create_fields_string = String::new();

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

        create_fields_string.push_str(&*format!("{} {} NOT NULL,", field_name, column_type))
    }

    create_fields_string.push_str("PRIMARY KEY (id)");

    let ident = derive_input.ident;
    let ident_str = ident.to_string();

    let create_string = format!("CREATE TABLE public.{}({});", ident_str, create_fields_string);

    let output = quote! {
        #[crash_orm::async_trait::async_trait]
        impl crash_orm::Schema for #ident {
            async fn create_table(connection: &crash_orm::DatabaseConnection) -> crash_orm::Result<()> {
                connection.execute(#create_string, &[]).await?;

                Ok(())
            }

            async fn drop_table(connection: &crash_orm::DatabaseConnection) -> crash_orm::Result<()> {
                Ok(())
            }

            async fn truncate_table(connection: &crash_orm::DatabaseConnection) -> crash_orm::Result<()> {
                Ok(())
            }
        }
    };

    output.into()
}