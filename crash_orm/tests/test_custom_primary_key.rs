use crash_orm::prelude::*;
use crash_orm_test::{default_create_table, setup_test_connection};

#[derive(Entity, Debug, Schema)]
pub struct CustomPrimaryKey {
    #[primary_key]
    custom_id: u32,
    test_field: String,
}

#[tokio::test]
async fn test_custom_primary_key() {
    let conn = setup_test_connection().await;
    default_create_table!(CustomPrimaryKey, conn);

    let entity = CustomPrimaryKeyCreate {
        test_field: String::from("test123"),
    }.insert(&conn).await.unwrap();

    let result = CustomPrimaryKey::get_by_primary(&conn, entity.custom_id).await.unwrap().unwrap();
    assert_eq!(result.test_field, "test123");

    CustomPrimaryKey::drop_table(&conn).await.unwrap()
}