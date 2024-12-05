use crash_orm::prelude::*;
use crash_orm_test::{default_create_table, setup_test_connection};

#[derive(Entity, Debug, Schema)]
pub struct TestItemEmpty {
    id: Option<u32>,
}

#[tokio::test]
async fn test_empty() {
    let conn = setup_test_connection().await;

    default_create_table!(TestItemEmpty, conn);

    let mut item = TestItemEmpty { id: None };
    item.insert(&conn).await.unwrap();
    let mut entity = TestItemEmpty::get_by_primary(&conn, item.id.unwrap()).await.unwrap();
    entity.persist(&conn).await.unwrap();

    TestItemEmpty::drop_table(&conn).await.unwrap();
}