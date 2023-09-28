use tokio_postgres::NoTls;
use crash_orm::{DatabaseConnection, Entity, OneToOne, Schema};
use crash_orm_derive::{Entity, Schema};

pub async fn setup_test_connection() -> DatabaseConnection {
    DatabaseConnection::new("postgresql://crash_orm:postgres@localhost/crash_orm_test", NoTls).await.unwrap()
}

#[derive(Entity, Debug, Schema)]
pub struct TestItem19 {
    pub id: Option<u32>,
    pub name1: Option<String>,
    pub active: bool,
    pub other: Option<OneToOne<TestItem20>>,
}

#[derive(Entity, Debug, Schema)]
pub struct TestItem20 {
    pub id: Option<u32>,
    pub name1: Option<String>,
    pub active: bool,
}

impl TestItem19 {
    fn test2() -> Self {
        Self {
            id: None,
            name1: Some(String::from("test123")),
            active: true,
            other: Some(OneToOne::new(1)),
        }
    }
}

impl TestItem20 {
    fn test() -> Self {
        Self {
            id: None,
            name1: Some(String::from("Test1234")),
            active: false,
        }
    }
}

#[tokio::test]
async fn test_one_to_one() {
    let conn = setup_test_connection().await;

    if !TestItem20::table_exists(&conn).await.unwrap() {
        assert!(TestItem20::create_table(&conn).await.is_ok());
    } else {
        assert!(TestItem20::truncate_table(&conn).await.is_ok());
    }

    if !TestItem19::table_exists(&conn).await.unwrap() {
        assert!(TestItem19::create_table(&conn).await.is_ok());
    } else {
        assert!(TestItem19::truncate_table(&conn).await.is_ok());
    }

    TestItem20::test().persist(&conn).await.unwrap();
    TestItem19::test2().persist(&conn).await.unwrap();

    let results = TestItem19::query()
        .execute(&conn).await;
    assert!(results.is_ok());
    let results = results.unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].other.as_ref().unwrap().get(&conn).await.unwrap().name1, Some(String::from("Test1234")));

    assert!(TestItem19::drop_table(&conn).await.is_ok());
    assert!(TestItem20::drop_table(&conn).await.is_ok());
}