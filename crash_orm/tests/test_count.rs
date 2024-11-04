use crash_orm::prelude::*;
use crash_orm_test::setup_test_connection;
use tokio_postgres::Row;

#[derive(Entity, Debug, Schema)]
pub struct TestItem6 {
    pub id: Option<u32>,
    pub name1: Option<String>,
    pub name2: Option<String>,
    pub number: Option<i32>,
}

impl TestItem6 {
    fn test() -> Self {
        Self {
            id: None,
            name1: Some(String::from("test123")),
            name2: None,
            number: Some(1),
        }
    }

    fn test2() -> Self {
        Self {
            id: None,
            name1: Some(String::from("test123")),
            name2: Some(String::from("1234")),
            number: None,
        }
    }

    fn test3() -> Self {
        Self {
            id: None,
            name1: Some(String::from("test1234")),
            name2: Some(String::from("1234")),
            number: None,
        }
    }
}

#[tokio::test]
async fn test_count() {
    let conn = setup_test_connection().await;

    if !TestItem6::table_exists(&conn).await.unwrap() {
        assert!(TestItem6::create_table(&conn).await.is_ok());
    } else {
        assert!(TestItem6::truncate_table(&conn).await.is_ok());
    }

    assert!(vec![TestItem6::test(), TestItem6::test2(), TestItem6::test3()]
        .persist_all(&conn)
        .await
        .is_ok());

    let result = TestItem6::select_query::<Row>(&[&TestItem6Column::NAME2.count_column(false)])
        .condition(TestItem6Column::NUMBER.is_null())
        .fetch_single(&conn).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().get::<usize, i64>(0), 2);

    let result = TestItem6::select_query::<Row>(&[&TestItem6Column::NAME2.count_column(true)])
        .condition(TestItem6Column::NUMBER.is_null())
        .fetch_single(&conn).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().get::<usize, i64>(0), 1);

    let result = TestItem6::count(&conn).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 3);

    assert!(TestItem6::drop_table(&conn).await.is_ok());
}
