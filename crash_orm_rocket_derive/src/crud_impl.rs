use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};

pub fn crud_impl(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    let ident = derive_input.ident;
    let create_ident = Ident::new(&*format!("{}Create", ident), ident.span());
    let mod_ident = Ident::new(&*ident.to_string().to_lowercase(), ident.span());

    let output = quote! {
        mod #mod_ident {
            use super::{#ident, #create_ident};

            #[rocket::post("/create", data = "<json>")]
            pub async fn create(json: rocket::serde::json::Json<#create_ident>, conn: &rocket::State<crash_orm::connection::CrashOrmDatabaseConnection>) -> rocket::serde::json::Json<u32> {
                use crash_orm::entity::CreateEntity;
                let element = json.0.insert(&**conn).await.unwrap();

                rocket::serde::json::Json(element.id)
            }

            #[rocket::get("/get/<id>")]
            pub async fn read(id: u32, conn: &rocket::State<crash_orm::connection::CrashOrmDatabaseConnection>) -> rocket::serde::json::Json<Option<#ident>> {
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
                if let Some(entity) = #ident::get_by_primary(&**conn, id).await.unwrap() {
                    entity.remove(&**conn).await.unwrap();
                }

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