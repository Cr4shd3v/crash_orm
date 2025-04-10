use crash_orm::prelude::{Entity, EntityCreateVec, NullQueryColumn, Schema, SingleResult, SumColumn};
use crash_orm_test::setup_test_connection;

#[derive(Entity, Debug, Schema)]
pub struct TestItem7 {
    pub id: u32,
    pub name1: Option<String>,
    pub name2: Option<String>,
    pub number: Option<i32>,
}

impl TestItem7Create {
    fn test() -> Self {
        Self {
            name1: Some(String::from("test123")),
            name2: None,
            number: Some(2),
        }
    }

    fn test2() -> Self {
        Self {
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

    vec![TestItem7Create::test(), TestItem7Create::test2()]
        .insert_all(&conn)
        .await
        .unwrap();

    let result = TestItem7::select_query::<SingleResult<i64>>(&[&TestItem7Column::NUMBER.sum(true)])
        .fetch_single(&conn).await;
    assert!(result.is_ok());
    assert_eq!(*result.unwrap().unwrap(), 17);

    let result = TestItem7::select_query::<SingleResult<i64>>(&[&TestItem7Column::NUMBER.sum(true)])
        .condition(TestItem7Column::NAME2.is_not_null())
        .fetch_single(&conn).await;
    assert!(result.is_ok());
    assert_eq!(*result.unwrap().unwrap(), 15);

    assert!(TestItem7::drop_table(&conn).await.is_ok());
}
