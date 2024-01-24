use crash_orm::{BoolQueryColumn, CrashOrmDatabaseConnection, Entity, EntityVec, Schema};
use crash_orm_derive::{Entity, Schema};
use tokio_postgres::NoTls;

pub async fn setup_test_connection() -> CrashOrmDatabaseConnection {
    CrashOrmDatabaseConnection::new(
        "postgresql://crash_orm:postgres@localhost/crash_orm_test",
        NoTls,
    )
    .await
    .unwrap()
}

#[derive(Entity, Debug, Schema)]
pub struct TestItem13 {
    pub id: Option<u32>,
    pub name1: Option<String>,
    pub active: bool,
    pub number: Option<i32>,
}

impl TestItem13 {
    fn test() -> Self {
        Self {
            id: None,
            name1: Some(String::from("test1234")),
            active: false,
            number: Some(441),
        }
    }

    fn test2() -> Self {
        Self {
            id: None,
            name1: Some(String::from("test123")),
            active: true,
            number: Some(440),
        }
    }
}

#[tokio::test]
async fn test_bool() {
    let conn = setup_test_connection().await;

    if !TestItem13::table_exists(&conn).await.unwrap() {
        assert!(TestItem13::create_table(&conn).await.is_ok());
    } else {
        assert!(TestItem13::truncate_table(&conn).await.is_ok());
    }

    vec![TestItem13::test(), TestItem13::test2()]
        .persist_all(&conn)
        .await
        .unwrap();

    let results = TestItem13::query()
        .condition(TestItem13Column::ACTIVE.is_true())
        .execute(&conn)
        .await;
    assert!(results.is_ok());
    assert_eq!(results.unwrap().len(), 1);

    let results = TestItem13::query()
        .condition(TestItem13Column::ACTIVE.is_false())
        .execute(&conn)
        .await;
    assert!(results.is_ok());
    assert_eq!(results.unwrap().len(), 1);

    assert!(TestItem13::drop_table(&conn).await.is_ok());
}
