use tokio_postgres::NoTls;
use uuid::Uuid;

use crash_orm::{BaseColumn, CrashOrmDatabaseConnection, Entity, EqualQueryColumn, Schema};
use crash_orm_derive::{Entity, Schema};

pub async fn setup_test_connection() -> CrashOrmDatabaseConnection {
    CrashOrmDatabaseConnection::new(
        "postgresql://crash_orm:postgres@localhost/crash_orm_test",
        NoTls,
    )
        .await
        .unwrap()
}

#[derive(Entity, Debug, Schema)]
pub struct TestItemUuid {
    pub id: Option<Uuid>,
    pub test: u32,
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