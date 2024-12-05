use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Ident};

pub fn crud_impl(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    let Data::Struct(struct_data) = derive_input.data else {
        panic!("only structs are supported")
    };

    let ident = derive_input.ident;
    let mut field_mapping = quote! {};
    let mut create_fields = quote! {};

    for field in struct_data.fields {
        let field_ident = field.ident.unwrap();
        let field_ident_str = field_ident.to_string();
        let field_type = field.ty;

        if field_ident_str != "id" {
            field_mapping.extend(quote! {
                #field_ident: json.0.#field_ident,
            });

            create_fields.extend(quote! {
                pub #field_ident: #field_type,
            });
        }
    }

    let create_ident = Ident::new(&*format!("{}Create", ident), ident.span());
    let mod_ident = Ident::new(&*ident.to_string().to_lowercase(), ident.span());

    let output = quote! {
        mod #mod_ident {
            use super::#ident;

            #[derive(serde::Deserialize, serde::Serialize)]
            pub struct #create_ident {
                #create_fields
            }

            #[rocket::post("/create", data = "<json>")]
            pub async fn create(json: rocket::serde::json::Json<#create_ident>, conn: &rocket::State<crash_orm::connection::CrashOrmDatabaseConnection>) -> rocket::serde::json::Json<u32> {
                let mut element = #ident {
                    id: None,
                    #field_mapping
                };

                use crash_orm::entity::Entity;
                element.insert(&**conn).await.unwrap();

                rocket::serde::json::Json(element.id.unwrap())
            }

            #[rocket::get("/get/<id>")]
            pub async fn read(id: u32, conn: &rocket::State<crash_orm::connection::CrashOrmDatabaseConnection>) -> rocket::serde::json::Json<#ident> {
                use crash_orm::entity::PrimaryKeyEntity;
                rocket::serde::json::Json(#ident::get_by_primary(&**conn, id).await.unwrap())
            }

            #[rocket::post("/update", data = "<json>")]
            pub async fn update(json: rocket::serde::json::Json<#ident>, conn: &rocket::State<crash_orm::connection::CrashOrmDatabaseConnection>) -> rocket::serde::json::Json<bool> {
                use crash_orm::entity::Entity;
                json.0.update(&**conn).await.unwrap();

                rocket::serde::json::Json(true)
            }

            #[rocket::delete("/delete/<id>")]
            pub async fn delete(id: u32, conn: &rocket::State<crash_orm::connection::CrashOrmDatabaseConnection>) -> rocket::serde::json::Json<bool> {
                use crash_orm::entity::{Entity, PrimaryKeyEntity};
                #ident::get_by_primary(&**conn, id).await.unwrap().remove(&**conn).await.unwrap();

                rocket::serde::json::Json(true)
            }
        }

        impl #ident {
            pub fn get_crud_routes() -> Vec<rocket::Route> {
                rocket::routes![#mod_ident::create, #mod_ident::read, #mod_ident::update, #mod_ident::delete]
            }
        }
    };

    output.into()
}