use crash_orm::prelude::{
    Entity, EntityVec, MaxColumn, MinColumn, NullQueryColumn, Schema,
};
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

    let result = TestItem8Column::NUMBER.min(&conn).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().unwrap(), 2);

    let result = TestItem8Column::NUMBER
        .min_query(&conn, TestItem8Column::NAME2.is_not_null())
        .await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().unwrap(), 15);

    let result = TestItem8Column::NUMBER.max(&conn).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().unwrap(), 15);

    let result = TestItem8Column::NUMBER
        .max_query(&conn, TestItem8Column::NAME2.is_null())
        .await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().unwrap(), 2);

    assert!(TestItem8::drop_table(&conn).await.is_ok());
}
