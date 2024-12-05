use uuid::Uuid;

use crash_orm::prelude::*;
use crash_orm_test::setup_test_connection;

#[derive(Entity, Debug, Schema)]
pub struct TestItemUuid {
    pub id: Uuid,
    pub test: u32,
}

#[derive(Entity, Debug, Schema)]
pub struct TestItemI32 {
    pub id: i32,
    pub t: u32,
    pub test_item_uuid: OneToOne<TestItemUuid, Uuid>,
}

#[tokio::test]
async fn test_uuid() {
    let conn = setup_test_connection().await;

    if !TestItemUuid::table_exists(&conn).await.unwrap() {
        assert!(TestItemUuid::create_table(&conn).await.is_ok());
    } else {
        assert!(TestItemUuid::truncate_table(&conn).await.is_ok());
    }

    // Automatically generate a uuid v4 through selected feature flag
    let entity = TestItemUuidCreate {
        test: 111,
    }.insert(&conn).await.unwrap();

    let result = TestItemUuid::query()
        .condition(TestItemUuidColumn::ID.equals(entity.id))
        .fetch(&conn).await.unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].test, 111);

    assert!(TestItemUuid::drop_table(&conn).await.is_ok());
}