use uuid::Uuid;

use crash_orm::{BaseColumn, Entity, EqualQueryColumn, OneToOne, Schema};
use crash_orm_derive::{Entity, Schema};
use crash_orm_test::setup_test_connection;

#[derive(Entity, Debug, Schema)]
pub struct TestItemUuid {
    pub id: Option<Uuid>,
    pub test: u32,
}

#[derive(Entity, Debug, Schema)]
pub struct TestItemI32 {
    pub id: Option<Uuid>,
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

    let uuid = Uuid::now_v7();
    let entity = TestItemUuid {
        id: Some(uuid),
        test: 111,
    };

    assert!(entity.insert_get_id(&conn).await.is_ok());

    let result = TestItemUuid::query()
        .condition(TestItemUuidColumn::ID.equals(uuid))
        .execute(&conn).await.unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].test, 111);

    assert!(TestItemUuid::drop_table(&conn).await.is_ok());
}