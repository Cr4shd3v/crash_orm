use crash_orm::{DatabaseConnection, Entity, EntityVec, EqualQueryColumn, QueryEntity, Schema};
use crash_orm_derive::{Entity, Query, Schema};

#[derive(Entity, Debug)]
pub struct TestItem1 {
    pub id: Option<u32>,
    pub name: String,
}

impl TestItem1 {
    fn test() -> Self {
        Self {
            id: None,
            name: String::from("test123"),
        }
    }
}

pub async fn setup_test_connection() -> DatabaseConnection {
    DatabaseConnection::new("postgresql://crash_orm:postgres@localhost/crash_orm_test").await.unwrap()
}

#[tokio::test]
async fn test_basic() {
    let conn = setup_test_connection().await;

    let mut item = TestItem1::test();

    item.insert_set_id(&conn).await.unwrap();
    assert!(item.id.is_some());
    let item = TestItem1::get_by_id(&conn, item.id.unwrap()).await;
    assert!(item.is_ok());
    let mut item = item.unwrap();
    println!("{:?}", item);
    assert_eq!(&*item.name, "test123");

    // cleanup
    item.remove(&conn).await.unwrap();
}

#[tokio::test]
async fn test_persist() {
    let conn = setup_test_connection().await;

    let mut item = TestItem1::test();
    item.persist(&conn).await.unwrap();
    assert!(item.id.is_some());
    item.name = String::from("test_updated");
    item.persist(&conn).await.unwrap();
    let item_from_db = TestItem1::get_by_id(&conn, item.id.unwrap()).await;
    assert!(item_from_db.is_ok());
    let item_from_db = item_from_db.unwrap();
    assert_eq!(&*item_from_db.name, "test_updated");
    assert_eq!(item_from_db.id, item.id);

    // cleanup
    item.remove(&conn).await.unwrap();
}

#[derive(Entity, Debug)]
pub struct TestItem2 {
    pub id: Option<u32>,
    pub name: String,
}

impl TestItem2 {
    fn test() -> Self {
        Self {
            id: None,
            name: String::from("test123"),
        }
    }
}

#[tokio::test]
async fn test_get_all() {
    let conn = setup_test_connection().await;
    vec![TestItem2::test(), TestItem2::test(), TestItem2::test()].persist_all(&conn).await.unwrap();

    let results = TestItem2::get_all(&conn).await;
    assert!(results.is_ok());
    let mut results = results.unwrap();
    assert_eq!(results.len(), 3);

    // cleanup
    results.remove_all(&conn).await.unwrap();
}

#[derive(Entity, Debug, Schema)]
pub struct TestItem3 {
    pub id: Option<u32>,
    pub name: String,
}

impl TestItem3 {
    fn test() -> Self {
        Self {
            id: None,
            name: String::from("test123"),
        }
    }
}

#[tokio::test]
async fn test_schema() {
    let conn = setup_test_connection().await;

    assert_eq!(TestItem3::TABLE_NAME, "testitem3");
    assert!(TestItem3::create_table(&conn).await.is_ok());
    assert!(TestItem3::test().persist(&conn).await.is_ok());
    let all = TestItem3::get_all(&conn).await;
    assert!(all.is_ok());
    assert_eq!(all.unwrap().len(), 1);
    assert!(TestItem3::truncate_table(&conn).await.is_ok());
    let all = TestItem3::get_all(&conn).await;
    assert!(all.is_ok());
    assert_eq!(all.unwrap().len(), 0);
    let exists = TestItem3::table_exists(&conn).await;
    assert!(exists.is_ok());
    assert!(exists.unwrap());
    assert!(TestItem3::drop_table(&conn).await.is_ok());
    assert!(TestItem3::test().persist(&conn).await.is_err());
    let exists = TestItem3::table_exists(&conn).await;
    assert!(exists.is_ok());
    assert!(!exists.unwrap());
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
async fn test_query() {
    let conn = setup_test_connection().await;

    assert!(TestItem4::create_table(&conn).await.is_ok());

    assert!(TestItem4::test().persist(&conn).await.is_ok());
    assert!(TestItem4::test2().persist(&conn).await.is_ok());
    let results = TestItem4::query(&conn, TestItem4Column::NAME.equals(String::from("test123"))).await;
    println!("{:?}", results);
    assert!(results.is_ok());
    let results = results.unwrap();
    assert_eq!(results.len(), 1);

    assert!(TestItem4::drop_table(&conn).await.is_ok());
}