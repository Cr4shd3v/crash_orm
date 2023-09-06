use tokio_postgres::NoTls;
use crash_orm::{DatabaseConnection, Entity, EntityVec, ManyToOne, Schema};
use crash_orm_derive::{Entity, Schema};

pub async fn setup_test_connection() -> DatabaseConnection {
    DatabaseConnection::new("postgresql://crash_orm:postgres@localhost/crash_orm_test", NoTls).await.unwrap()
}

#[derive(Entity, Debug, Schema)]
pub struct TestItem21 {
    pub id: Option<u32>,
    pub name1: Option<String>,
    pub active: bool,
    pub other: ManyToOne<TestItem22>,
}

#[derive(Entity, Debug, Schema)]
pub struct TestItem22 {
    pub id: Option<u32>,
    pub name1: Option<String>,
    pub active: bool,
}

impl TestItem21 {
    fn test() -> Self {
        Self {
            id: None,
            name1: Some(String::from("test123")),
            active: false,
            other: ManyToOne::new(1),
        }
    }

    fn test2() -> Self {
        Self {
            id: None,
            name1: Some(String::from("test321")),
            active: true,
            other: ManyToOne::new(1),
        }
    }
}

impl TestItem22 {
    fn test() -> Self {
        Self {
            id: None,
            name1: Some(String::from("Test1234")),
            active: false,
        }
    }
}

#[tokio::test]
async fn test_many_to_one() {
    let conn = setup_test_connection().await;

    if !TestItem22::table_exists(&conn).await.unwrap() {
        assert!(TestItem22::create_table(&conn).await.is_ok());
    } else {
        assert!(TestItem22::truncate_table(&conn).await.is_ok());
    }

    if !TestItem21::table_exists(&conn).await.unwrap() {
        assert!(TestItem21::create_table(&conn).await.is_ok());
    } else {
        assert!(TestItem21::truncate_table(&conn).await.is_ok());
    }

    TestItem22::test().persist(&conn).await.unwrap();
    vec![TestItem21::test(), TestItem21::test2()].persist_all(&conn).await.unwrap();

    let results = TestItem21::query()
        .execute(&conn).await;
    assert!(results.is_ok());
    let mut results = results.unwrap();
    assert_eq!(results.len(), 2);
    let result1 = &mut results[0].other;
    assert_eq!(result1.get(&conn).await.unwrap().name1, Some(String::from("Test1234")));
    let result2 = &mut results[1].other;
    assert_eq!(result2.get(&conn).await.unwrap().name1, Some(String::from("Test1234")));

    assert!(TestItem21::drop_table(&conn).await.is_ok());
    assert!(TestItem22::drop_table(&conn).await.is_ok());
}