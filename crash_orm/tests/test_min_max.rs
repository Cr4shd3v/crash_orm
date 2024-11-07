use crash_orm::prelude::{Entity, EntityVec, MaxColumn, MinColumn, NullQueryColumn, Schema, SingleResult};
use crash_orm_test::setup_test_connection;

#[derive(Entity, Debug, Schema)]
pub struct TestItem8 {
    pub id: Option<u32>,
    pub name1: Option<String>,
    pub name2: Option<String>,
    pub number: Option<i32>,
}

impl TestItem8 {
    fn test() -> Self {
        Self {
            id: None,
            name1: Some(String::from("test123")),
            name2: None,
            number: Some(2),
        }
    }

    fn test2() -> Self {
        Self {
            id: None,
            name1: Some(String::from("test123")),
            name2: Some(String::from("1234")),
            number: Some(15),
        }
    }
}

#[tokio::test]
async fn test_min_max() {
    let conn = setup_test_connection().await;

    if !TestItem8::table_exists(&conn).await.unwrap() {
        assert!(TestItem8::create_table(&conn).await.is_ok());
    } else {
        assert!(TestItem8::truncate_table(&conn).await.is_ok());
    }

    vec![TestItem8::test(), TestItem8::test2()]
        .persist_all(&conn)
        .await
        .unwrap();

    let result = TestItem8::select_query::<SingleResult<i32>>(&[&TestItem8Column::NUMBER.min()])
        .fetch_single(&conn).await;
    assert!(result.is_ok());
    assert_eq!(*result.unwrap(), 2);

    let result = TestItem8::select_query::<SingleResult<i32>>(&[&TestItem8Column::NUMBER.min()])
        .condition(TestItem8Column::NAME2.is_not_null())
        .fetch_single(&conn).await;
    assert!(result.is_ok());
    assert_eq!(*result.unwrap(), 15);

    let result = TestItem8::select_query::<SingleResult<i32>>(&[&TestItem8Column::NUMBER.max()])
        .fetch_single(&conn).await;
    assert!(result.is_ok());
    assert_eq!(*result.unwrap(), 15);

    let result = TestItem8::select_query::<SingleResult<i32>>(&[&TestItem8Column::NUMBER.max()])
        .condition(TestItem8Column::NAME2.is_null())
        .fetch_single(&conn).await;
    assert!(result.is_ok());
    assert_eq!(*result.unwrap(), 2);

    assert!(TestItem8::drop_table(&conn).await.is_ok());
}
