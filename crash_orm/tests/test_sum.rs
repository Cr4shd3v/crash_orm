use crash_orm::{EntityVec, NullQueryColumn, Schema, SumColumn};
use crash_orm_derive::{Entity, Schema};
use crash_orm_test::setup_test_connection;

#[derive(Entity, Debug, Schema)]
pub struct TestItem7 {
    pub id: Option<u32>,
    pub name1: Option<String>,
    pub name2: Option<String>,
    pub number: Option<i32>,
}

impl TestItem7 {
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
async fn test_sum() {
    let conn = setup_test_connection().await;

    if !TestItem7::table_exists(&conn).await.unwrap() {
        assert!(TestItem7::create_table(&conn).await.is_ok());
    } else {
        assert!(TestItem7::truncate_table(&conn).await.is_ok());
    }

    vec![TestItem7::test(), TestItem7::test2()]
        .persist_all(&conn)
        .await
        .unwrap();

    let result = TestItem7Column::NUMBER.sum(&conn, true).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 17);

    let result = TestItem7Column::NUMBER
        .sum_query(&conn, true, TestItem7Column::NAME2.is_not_null())
        .await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 15);

    assert!(TestItem7::drop_table(&conn).await.is_ok());
}
