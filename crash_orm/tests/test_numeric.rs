use crash_orm::prelude::{Entity, EntityCreateVec, MaxColumn, MinColumn, Schema};
use crash_orm_test::setup_test_connection;
use rust_decimal::Decimal;
use tokio_postgres::Row;

#[derive(Entity, Debug, Schema)]
pub struct TestItem9 {
    pub id: u32,
    pub name1: Option<String>,
    pub name2: Option<String>,
    pub number: Option<Decimal>,
}

impl TestItem9Create {
    fn test() -> Self {
        Self {
            name1: Some(String::from("test123")),
            name2: None,
            number: Some(Decimal::new(3200, 3)),
        }
    }

    fn test2() -> Self {
        Self {
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

    vec![TestItem9Create::test(), TestItem9Create::test2()]
        .insert_all(&conn)
        .await
        .unwrap();

    let result = TestItem9::select_query::<Row>(&[&TestItem9Column::NUMBER.max()])
        .fetch_single(&conn).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().get::<_, Decimal>(0), Decimal::new(3200, 3));

    let result = TestItem9::select_query::<Row>(&[&TestItem9Column::NUMBER.min()])
        .fetch_single(&conn).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().get::<_, Decimal>(0), Decimal::new(800, 3));

    assert!(TestItem9::drop_table(&conn).await.is_ok());
}
