use crash_orm::prelude::*;
use crash_orm_test::{default_create_table, setup_test_connection};

#[derive(Entity, Schema, Debug)]
pub struct TestItemReservedKeywords {
    pub id: Option<u32>,
    pub all: bool,
    pub references: String,
    pub order: i32,
}

#[tokio::test]
async fn test_reserved_keywords() {
    let conn = setup_test_connection().await;
    default_create_table!(TestItemReservedKeywords, conn);

    TestItemReservedKeywords {
        id: None,
        all: true,
        references: "test".to_string(),
        order: 1,
    }.insert(&conn).await.unwrap();

    TestItemReservedKeywords::drop_table(&conn).await.unwrap();
}
