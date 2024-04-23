use rust_decimal::Decimal;

use crash_orm::{EntityVec, MaxColumn, MinColumn, Schema};
use crash_orm_derive::{Entity, Schema};
use crash_orm_test::setup_test_connection;

#[derive(Entity, Debug, Schema)]
pub struct TestItem9 {
    pub id: Option<u32>,
    pub name1: Option<String>,
    pub name2: Option<String>,
    pub number: Option<Decimal>,
}

impl TestItem9 {
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
async fn test_decimal() {
    let conn = setup_test_connection().await;

    if !TestItem9::table_exists(&conn).await.unwrap() {
        assert!(TestItem9::create_table(&conn).await.is_ok());
    } else {
        assert!(TestItem9::truncate_table(&conn).await.is_ok());
    }

    vec![TestItem9::test(), TestItem9::test2()]
        .persist_all(&conn)
        .await
        .unwrap();

    let result = TestItem9Column::NUMBER.max(&conn).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().unwrap(), Decimal::new(3200, 3));

    let result = TestItem9Column::NUMBER.min(&conn).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().unwrap(), Decimal::new(800, 3));

    assert!(TestItem9::drop_table(&conn).await.is_ok());
}
