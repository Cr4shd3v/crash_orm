use crash_orm::prelude::{CompareQueryColumn, Entity, EntityCreateVec, OrderDirection, Schema};
use crash_orm_test::setup_test_connection;

#[derive(Entity, Debug, Schema)]
pub struct TestItem17 {
    pub id: u32,
    pub name1: Option<String>,
    pub active: bool,
    pub number: Option<i32>,
}

impl TestItem17Create {
    fn test() -> Self {
        Self {
            name1: Some(String::from("Test1234")),
            active: false,
            number: Some(441),
        }
    }

    fn test2() -> Self {
        Self {
            name1: Some(String::from("test123")),
            active: true,
            number: Some(440),
        }
    }
}

#[tokio::test]
async fn test_order() {
    let conn = setup_test_connection().await;

    if !TestItem17::table_exists(&conn).await.unwrap() {
        assert!(TestItem17::create_table(&conn).await.is_ok());
    } else {
        assert!(TestItem17::truncate_table(&conn).await.is_ok());
    }

    vec![TestItem17Create::test(), TestItem17Create::test2()]
        .insert_all(&conn)
        .await
        .unwrap();

    let results = TestItem17::query()
        .order(&TestItem17Column::NUMBER, OrderDirection::ASC)
        .fetch(&conn)
        .await;
    println!("{:?}", results);
    assert!(results.is_ok());
    let results = results.unwrap();
    assert_eq!(results.len(), 2);
    assert_eq!(results[0].number.unwrap(), 440);
    assert_eq!(results[1].number.unwrap(), 441);

    let results = TestItem17::query()
        .condition(TestItem17Column::NUMBER.greater_equal(400))
        .order(&TestItem17Column::NUMBER, OrderDirection::DESC)
        .fetch(&conn)
        .await;
    assert!(results.is_ok());
    let results = results.unwrap();
    assert_eq!(results.len(), 2);
    assert_eq!(results[0].number.unwrap(), 441);
    assert_eq!(results[1].number.unwrap(), 440);

    assert!(TestItem17::drop_table(&conn).await.is_ok());
}
