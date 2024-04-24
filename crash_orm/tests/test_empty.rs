use crash_orm::{Schema, Entity};
use crash_orm_derive::{Entity, Schema};
use crash_orm_test::{default_create_table, setup_test_connection};

#[derive(Entity, Debug, Schema)]
pub struct TestItemEmpty {
    id: Option<u32>,
}

#[tokio::test]
async fn test_empty() {
    let conn = setup_test_connection().await;

    default_create_table!(TestItemEmpty, conn);

    let item = TestItemEmpty { id: None };
    let id = item.insert_get_id(&conn).await.unwrap();
    let mut entity = TestItemEmpty::get_by_id(&conn, id).await.unwrap();
    entity.persist(&conn).await.unwrap();

    TestItemEmpty::drop_table(&conn).await.unwrap();
}