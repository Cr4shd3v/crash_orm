use crash_orm::prelude::{CompareQueryColumn, Entity, EntityCreateVec, Schema};
use crash_orm_test::setup_test_connection;

#[derive(Entity, Debug, Schema)]
pub struct TestItem12 {
    pub id: u32,
    pub name1: Option<String>,
    pub name2: Option<String>,
    pub number: Option<i32>,
}

impl TestItem12Create {
    fn test() -> Self {
        Self {
            name1: Some(String::from("test1234")),
            name2: None,
            number: Some(441),
        }
    }

    fn test2() -> Self {
        Self {
            name1: Some(String::from("test123")),
            name2: Some(String::from("1234")),
            number: Some(440),
        }
    }
}

#[tokio::test]
async fn test_compare() {
    let conn = setup_test_connection().await;

    if !TestItem12::table_exists(&conn).await.unwrap() {
        assert!(TestItem12::create_table(&conn).await.is_ok());
    } else {
        assert!(TestItem12::truncate_table(&conn).await.is_ok());
    }

    vec![TestItem12Create::test(), TestItem12Create::test2()]
        .insert_all(&conn)
        .await
        .unwrap();

    let results = TestItem12::query()
        .condition(TestItem12Column::NUMBER.greater_than(440))
        .fetch(&conn)
        .await;
    assert!(results.is_ok());
    assert_eq!(results.unwrap().len(), 1);

    let results = TestItem12::query()
        .condition(TestItem12Column::NUMBER.greater_equal(440))
        .fetch(&conn)
        .await;
    assert!(results.is_ok());
    assert_eq!(results.unwrap().len(), 2);

    let results = TestItem12::query()
        .condition(TestItem12Column::NUMBER.less_than(441))
        .fetch(&conn)
        .await;
    assert!(results.is_ok());
    assert_eq!(results.unwrap().len(), 1);

    let results = TestItem12::query()
        .condition(TestItem12Column::NUMBER.less_equal(441))
        .fetch(&conn)
        .await;
    assert!(results.is_ok());
    assert_eq!(results.unwrap().len(), 2);

    let results = TestItem12::query()
        .condition(TestItem12Column::NUMBER.between(0, 440))
        .fetch(&conn)
        .await;
    assert!(results.is_ok());
    assert_eq!(results.unwrap().len(), 1);

    let results = TestItem12::query()
        .condition(TestItem12Column::NUMBER.not_between(0, 440))
        .fetch(&conn)
        .await;
    assert!(results.is_ok());
    assert_eq!(results.unwrap().len(), 1);

    assert!(TestItem12::drop_table(&conn).await.is_ok());
}
