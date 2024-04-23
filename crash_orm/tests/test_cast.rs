use tokio_postgres::NoTls;

use crash_orm::{
    CrashOrmDatabaseConnection, Entity, EntityVec, EqualQueryColumn, Schema, TextCastVirtualColumn,
};
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
pub struct TestItem18 {
    pub id: Option<u32>,
    pub name1: Option<String>,
    pub active: bool,
    pub number: Option<i32>,
}

impl TestItem18 {
    fn test() -> Self {
        Self {
            id: None,
            name1: Some(String::from("Test1234")),
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
async fn test_cast() {
    let conn = setup_test_connection().await;

    if !TestItem18::table_exists(&conn).await.unwrap() {
        assert!(TestItem18::create_table(&conn).await.is_ok());
    } else {
        assert!(TestItem18::truncate_table(&conn).await.is_ok());
    }

    vec![TestItem18::test(), TestItem18::test2()]
        .persist_all(&conn)
        .await
        .unwrap();

    let results = TestItem18::query()
        .condition(
            TestItem18Column::NUMBER
                .cast_to_text()
                .equals(&String::from("440")),
        )
        .execute(&conn)
        .await;
    assert!(results.is_ok());
    assert_eq!(results.unwrap().len(), 1);

    assert!(TestItem18::drop_table(&conn).await.is_ok());
}
