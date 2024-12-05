use rocket::http::Status;
use rocket::local::asynchronous::Client;
use serde::{Deserialize, Serialize};
use crash_orm::prelude::*;
use crash_orm_rocket::CrashOrmDatabaseFairing;
use crash_orm_rocket_derive::CRUD;
use crash_orm_test::TEST_DB_URL;
use crate::testcrud::TestCrudCreate;

#[tokio::test]
async fn test_crud_operation() {
    let rocket = rocket::build()
        .attach(CrashOrmDatabaseFairing::from_url(TEST_DB_URL))
        .mount("/test", TestCrud::get_crud_routes())
        .ignite().await.unwrap();

    let conn = rocket.state::<CrashOrmDatabaseConnection>().unwrap();
    TestCrud::create_table_if_not_exists(conn).await.unwrap();

    let client = Client::tracked(rocket).await.unwrap();
    let response = client.post("/test/create").json(&TestCrudCreate {
        name: "test".to_string(),
    }).dispatch().await;
    assert_eq!(response.status(), Status::Ok);

    let id = response.into_json::<u32>().await.unwrap();

    let response = client.post("/test/update").json(&TestCrud {
        id: Some(id),
        name: "test123".to_string(),
    }).dispatch().await;
    assert_eq!(response.status(), Status::Ok);

    let response = client.get(format!("/test/get/{}", id)).dispatch().await;
    assert_eq!(response.status(), Status::Ok);
    let result = response.into_json::<TestCrud>().await.unwrap();
    assert_eq!(result.name, "test123");

    let response = client.delete(format!("/test/delete/{}", id)).dispatch().await;
    assert_eq!(response.status(), Status::Ok);
}

#[derive(Entity, Schema, Debug, Serialize, Deserialize, CRUD)]
struct TestCrud {
    id: Option<u32>,
    name: String,
}
