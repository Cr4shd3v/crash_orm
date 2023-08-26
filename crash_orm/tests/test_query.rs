use crash_orm::{DatabaseConnection, Entity, EqualQueryColumn, NullQueryColumn, QueryEntity, Schema};
use crash_orm_derive::{Entity, Query, Schema};

pub async fn setup_test_connection() -> DatabaseConnection {
    DatabaseConnection::new("postgresql://crash_orm:postgres@localhost/crash_orm_test").await.unwrap()
}

#[derive(Entity, Debug, Schema, Query)]
pub struct TestItem4 {
    pub id: Option<u32>,
    pub name: Option<String>,
}

impl TestItem4 {
    fn test() -> Self {
        Self {
            id: None,
            name: Some(String::from("test123")),
        }
    }

    fn test2() -> Self {
        Self {
            id: None,
            name: Some(String::from("test1234")),
        }
    }
}

#[tokio::test]
async fn test_query_simple() {
    let conn = setup_test_connection().await;

    if !TestItem4::table_exists(&conn).await.unwrap() {
        assert!(TestItem4::create_table(&conn).await.is_ok());
    } else {
        assert!(TestItem4::truncate_table(&conn).await.is_ok());
    }

    assert!(TestItem4::test().persist(&conn).await.is_ok());
    assert!(TestItem4::test2().persist(&conn).await.is_ok());
    let results = TestItem4::query(&conn, TestItem4Column::NAME.equals(String::from("test123"))).await;
    println!("{:?}", results);
    assert!(results.is_ok());
    let results = results.unwrap();
    assert_eq!(results.len(), 1);

    assert!(TestItem4::drop_table(&conn).await.is_ok());
}

#[derive(Entity, Debug, Schema, Query)]
pub struct TestItem5 {
    pub id: Option<u32>,
    pub name1: Option<String>,
    pub name2: Option<String>,
    pub number: Option<i32>,
}

impl TestItem5 {
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
}

#[tokio::test]
async fn test_query_complex() {
    let conn = setup_test_connection().await;

    if !TestItem5::table_exists(&conn).await.unwrap() {
        assert!(TestItem5::create_table(&conn).await.is_ok());
    } else {
        assert!(TestItem5::truncate_table(&conn).await.is_ok());
    }

    assert!(TestItem5::test().persist(&conn).await.is_ok());
    assert!(TestItem5::test2().persist(&conn).await.is_ok());

    let results = TestItem5::query(&conn, TestItem5Column::NAME1.equals(String::from("test123"))).await;
    println!("1: {:?}", results);
    assert!(results.is_ok());
    assert_eq!(results.unwrap().len(), 2);

    let results = TestItem5::query(
        &conn,
        TestItem5Column::NAME1.equals(String::from("test123")).and(TestItem5Column::NUMBER.is_null()),
    ).await;
    println!("2: {:?}", results);
    assert!(results.is_ok());
    assert_eq!(results.unwrap().len(), 1);

    let results = TestItem5::query(
        &conn,
        TestItem5Column::NUMBER.is_null().or(TestItem5Column::NAME2.is_null()),
    ).await;
    println!("3: {:?}", results);
    assert!(results.is_ok());
    assert_eq!(results.unwrap().len(), 2);

    assert!(TestItem5::drop_table(&conn).await.is_ok());
}