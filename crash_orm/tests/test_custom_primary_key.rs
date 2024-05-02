use crash_orm::{Entity, Schema};
use crash_orm_derive::{Entity, Schema};
use crash_orm_test::{default_create_table, setup_test_connection};

#[derive(Entity, Debug, Schema)]
pub struct CustomPrimaryKey {
    #[primary_key]
    custom_id: Option<u32>,
    test_field: String,
}

#[tokio::test]
async fn test_custom_primary_key() {
    let conn = setup_test_connection().await;
    default_create_table!(CustomPrimaryKey, conn);

    let entity = CustomPrimaryKey {
        custom_id: None,
        test_field: String::from("test123"),
    };
    let id = entity.insert_get_id(&conn).await.unwrap();
    let result = CustomPrimaryKey::get_by_primary(&conn, id).await.unwrap();
    assert_eq!(result.test_field, "test123");

    CustomPrimaryKey::drop_table(&conn).await.unwrap()
}