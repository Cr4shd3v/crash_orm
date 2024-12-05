use crash_orm::prelude::{AvgColumn, Entity, EntityCreateVec, NullQueryColumn, Schema, SingleResult};
use crash_orm_test::setup_test_connection;
use rust_decimal::Decimal;

#[derive(Entity, Debug, Schema)]
pub struct TestItem10 {
    pub id: u32,
    pub name1: Option<String>,
    pub name2: Option<String>,
    pub number: Option<Decimal>,
}

impl TestItem10Create {
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
async fn test_avg() {
    let conn = setup_test_connection().await;

    if !TestItem10::table_exists(&conn).await.unwrap() {
        assert!(TestItem10::create_table(&conn).await.is_ok());
    } else {
        assert!(TestItem10::truncate_table(&conn).await.is_ok());
    }

    vec![TestItem10Create::test(), TestItem10Create::test2()]
        .insert_all(&conn)
        .await
        .unwrap();

    let result = TestItem10::select_query::<SingleResult<Decimal>>(&[&TestItem10Column::NUMBER.avg(true)])
        .fetch_single(&conn).await;
    assert!(result.is_ok());
    assert_eq!(*result.unwrap(), Decimal::from(2));

    let result = TestItem10::select_query::<SingleResult<Decimal>>(&[&TestItem10Column::NUMBER.avg(true)])
        .condition(TestItem10Column::NAME2.is_null())
        .fetch_single(&conn).await;
    assert!(result.is_ok());
    assert_eq!(*result.unwrap(), Decimal::new(3200, 3));

    assert!(TestItem10::drop_table(&conn).await.is_ok());
}
