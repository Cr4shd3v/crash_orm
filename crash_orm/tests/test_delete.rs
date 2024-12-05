use crash_orm::prelude::*;
use crash_orm_test::{default_create_table, setup_test_connection};

#[derive(Entity, Debug, Schema)]
struct TestItemDelete {
    id: u32,
    name: String,
}

#[tokio::test]
async fn test_delete() {
    let conn = setup_test_connection().await;
    default_create_table!(TestItemDelete, conn);

    TestItemDeleteCreate {
        name: "test".to_string()
    }.insert(&conn).await.unwrap();

    TestItemDelete::delete()
        .condition(TestItemDeleteColumn::NAME.equals("test"))
        .execute(&conn).await.unwrap();

    assert_eq!(TestItemDelete::count(&conn).await.unwrap(), 0);

    TestItemDelete::drop_table(&conn).await.unwrap();
}
