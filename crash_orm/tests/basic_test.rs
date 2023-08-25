use crash_orm::{DatabaseConnection, Entity, EntityVec};
use crash_orm_derive::Entity;

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