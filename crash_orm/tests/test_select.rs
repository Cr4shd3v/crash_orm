use crash_orm::prelude::{BoolQueryColumn, Entity, EntityCreateVec, Schema, StringVirtualColumn};
use crash_orm_test::setup_test_connection;
use tokio_postgres::Row;

#[derive(Entity, Debug, Schema)]
pub struct TestItem16 {
    pub id: u32,
    pub name1: Option<String>,
    pub active: bool,
    pub number: Option<i32>,
}

impl TestItem16Create {
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
async fn test_select() {
    let conn = setup_test_connection().await;

    if !TestItem16::table_exists(&conn).await.unwrap() {
        assert!(TestItem16::create_table(&conn).await.is_ok());
    } else {
        assert!(TestItem16::truncate_table(&conn).await.is_ok());
    }

    vec![TestItem16Create::test(), TestItem16Create::test2()]
        .insert_all(&conn)
        .await
        .unwrap();

    let results = TestItem16::select_query::<Row>(&[&TestItem16Column::NUMBER])
        .fetch(&conn)
        .await;
    println!("{:?}", results);
    assert!(results.is_ok());
    let results = results.unwrap();
    assert_eq!(results.len(), 2);
    assert_eq!(results[0].len(), 1);

    let results = TestItem16::select_query::<Row>(&[
        &TestItem16Column::NUMBER,
        &TestItem16Column::NAME1,
        &TestItem16Column::ACTIVE,
    ])
    .fetch(&conn)
    .await;
    println!("{:?}", results);
    assert!(results.is_ok());
    let results = results.unwrap();
    assert_eq!(results.len(), 2);
    assert_eq!(results[0].len(), 3);

    let results = TestItem16::select_query::<Row>(&[
        &TestItem16Column::NUMBER,
        &TestItem16Column::NAME1.reverse(),
    ])
    .condition(TestItem16Column::ACTIVE.is_true())
    .fetch(&conn)
    .await;
    println!("{:?}", results);
    assert!(results.is_ok());
    let results = results.unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].len(), 2);
    assert_eq!(results[0].get::<usize, String>(1), String::from("321tset"));

    assert!(TestItem16::drop_table(&conn).await.is_ok());
}
