use rust_decimal::Decimal;
use tokio_postgres::NoTls;

use crash_orm::{AvgColumn, CrashOrmDatabaseConnection, EntityVec, NullQueryColumn, Schema};
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
pub struct TestItem10 {
    pub id: Option<u32>,
    pub name1: Option<String>,
    pub name2: Option<String>,
    pub number: Option<Decimal>,
}

impl TestItem10 {
    fn test() -> Self {
        Self {
            id: None,
            name1: Some(String::from("test123")),
            name2: None,
            number: Some(Decimal::new(3200, 3)),
        }
    }

    fn test2() -> Self {
        Self {
            id: None,
            name1: Some(String::from("test123")),
            name2: Some(String::from("1234")),
            number: Some(Decimal::new(800, 3)),
        }
    }
}

#[tokio::test]
async fn test_avg() {
    let conn = setup_test_connection().await;

    if !TestItem10::table_exists(&conn).await.unwrap() {
        assert!(TestItem10::create_table(&conn).await.is_ok());
    } else {
        assert!(TestItem10::truncate_table(&conn).await.is_ok());
    }

    vec![TestItem10::test(), TestItem10::test2()]
        .persist_all(&conn)
        .await
        .unwrap();

    let result = TestItem10Column::NUMBER.avg(&conn, true).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Decimal::from(2));

    let result = TestItem10Column::NUMBER
        .avg_query(&conn, true, TestItem10Column::NAME2.is_null())
        .await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Decimal::new(3200, 3));

    assert!(TestItem10::drop_table(&conn).await.is_ok());
}
