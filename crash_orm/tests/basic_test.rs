use crash_orm::{Entity, EntityVec, Schema};
use crash_orm_derive::{Entity, Schema};
use crash_orm_test::setup_test_connection;

#[derive(Entity, Debug, Schema)]
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

#[tokio::test]
async fn test_basic() {
    let conn = setup_test_connection().await;

    if !TestItem1::table_exists(&conn).await.unwrap() {
        TestItem1::create_table(&conn).await.unwrap();
    }

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

#[derive(Entity, Debug, Schema)]
pub struct TestItem01 {
    pub id: Option<u32>,
    pub name: String,
}

impl TestItem01 {
    fn test() -> Self {
        Self {
            id: None,
            name: String::from("test123"),
        }
    }
}

#[tokio::test]
async fn test_persist() {
    let conn = setup_test_connection().await;

    if !TestItem01::table_exists(&conn).await.unwrap() {
        TestItem01::create_table(&conn).await.unwrap();
    }

    let mut item = TestItem01::test();
    item.persist(&conn).await.unwrap();
    assert!(item.id.is_some());
    item.name = String::from("test_updated");
    assert!(item.persist(&conn).await.is_ok());
    let item_from_db = TestItem01::get_by_id(&conn, item.id.unwrap()).await;
    assert!(item_from_db.is_ok());
    let item_from_db = item_from_db.unwrap();
    assert_eq!(&*item_from_db.name, "test_updated");
    assert_eq!(item_from_db.id, item.id);

    // cleanup
    assert!(item.remove(&conn).await.is_ok());
}

#[derive(Entity, Debug, Schema)]
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

    if !TestItem2::table_exists(&conn).await.unwrap() {
        TestItem2::create_table(&conn).await.unwrap();
    }

    vec![TestItem2::test(), TestItem2::test(), TestItem2::test()]
        .insert_all(&conn)
        .await
        .unwrap();

    let results = TestItem2::get_all(&conn).await;
    assert!(results.is_ok());
    let results = results.unwrap();
    assert_eq!(results.len(), 3);

    // cleanup
    results.remove_all(&conn).await.unwrap();

    assert!(TestItem2::get_all(&conn).await.unwrap().is_empty());
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

    assert_eq!(TestItem3::TABLE_NAME, "test_item_3");
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
