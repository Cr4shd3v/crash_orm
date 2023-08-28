use crash_orm::{BoolQueryColumn, DatabaseConnection, Entity, EntityVec, Schema};
use crash_orm_derive::{Entity, Schema};

pub async fn setup_test_connection() -> DatabaseConnection {
    DatabaseConnection::new("postgresql://crash_orm:postgres@localhost/crash_orm_test").await.unwrap()
}

#[derive(Entity, Debug, Schema)]
pub struct TestItem16 {
    pub id: Option<u32>,
    pub name1: Option<String>,
    pub active: bool,
    pub number: Option<i32>,
}

impl TestItem16 {
    fn test() -> Self {
        Self {
            id: None,
            name1: Some(String::from("Test1234")),
            active: false,
            number: Some(441),
        }
    }

    fn test2() -> Self {
        Self {
            id: None,
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

    vec![TestItem16::test(), TestItem16::test2()].persist_all(&conn).await.unwrap();

    let results = TestItem16::select(&conn, &[&TestItem16Column::NUMBER]).await;
    println!("{:?}", results);
    assert!(results.is_ok());
    let results = results.unwrap();
    assert_eq!(results.len(), 2);
    assert_eq!(results[0].len(), 1);

    let results = TestItem16::select(
        &conn,
        &[&TestItem16Column::NUMBER, &TestItem16Column::NAME1, &TestItem16Column::ACTIVE]
    ).await;
    println!("{:?}", results);
    assert!(results.is_ok());
    let results = results.unwrap();
    assert_eq!(results.len(), 2);
    assert_eq!(results[0].len(), 3);

    let results = TestItem16::select_query(
        &conn,
        &[&TestItem16Column::NUMBER, &TestItem16Column::NAME1],
        TestItem16Column::ACTIVE.is_true(),
    ).await;
    println!("{:?}", results);
    assert!(results.is_ok());
    let results = results.unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].len(), 2);

    assert!(TestItem16::drop_table(&conn).await.is_ok());
}