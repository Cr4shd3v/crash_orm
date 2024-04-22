use crash_orm_derive::{Entity, Schema};
use tokio_postgres::NoTls;
use uuid::Uuid;
use crash_orm::{CrashOrmDatabaseConnection, Entity, Schema};

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

    let entity = TestItemUuid {
        id: Some(Uuid::now_v7()),
        test: 1,
    };

    assert!(entity.insert_get_id(&conn).await.is_ok());

    assert!(TestItemUuid::drop_table(&conn).await.is_ok());
}