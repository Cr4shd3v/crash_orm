use crash_orm::prelude::{BoolQueryColumn, Entity, EntityCreateVec, Schema};
use crash_orm_test::setup_test_connection;

#[derive(Entity, Debug, Schema)]
pub struct TestItem13 {
    pub id: u32,
    pub name1: Option<String>,
    pub active: bool,
    pub number: Option<i32>,
}

impl TestItem13Create {
    fn test() -> Self {
        Self {
            name1: Some(String::from("test1234")),
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
async fn test_bool() {
    let conn = setup_test_connection().await;

    if !TestItem13::table_exists(&conn).await.unwrap() {
        assert!(TestItem13::create_table(&conn).await.is_ok());
    } else {
        assert!(TestItem13::truncate_table(&conn).await.is_ok());
    }

    vec![TestItem13Create::test(), TestItem13Create::test2()]
        .insert_all(&conn)
        .await
        .unwrap();

    let results = TestItem13::query()
        .condition(TestItem13Column::ACTIVE.is_true())
        .fetch(&conn)
        .await;
    assert!(results.is_ok());
    assert_eq!(results.unwrap().len(), 1);

    let results = TestItem13::query()
        .condition(TestItem13Column::ACTIVE.is_false())
        .fetch(&conn)
        .await;
    assert!(results.is_ok());
    assert_eq!(results.unwrap().len(), 1);

    assert!(TestItem13::drop_table(&conn).await.is_ok());
}
