use crash_orm::{DatabaseConnection, Entity, EntityVec, EqualQueryColumn, LengthVirtualColumn, Schema};
use crash_orm_derive::{Entity, Schema};

pub async fn setup_test_connection() -> DatabaseConnection {
    DatabaseConnection::new("postgresql://crash_orm:postgres@localhost/crash_orm_test").await.unwrap()
}

#[derive(Entity, Debug, Schema)]
pub struct TestItem15 {
    pub id: Option<u32>,
    pub name1: Option<String>,
    pub active: bool,
    pub number: Option<i32>,
}

impl TestItem15 {
    fn test() -> Self {
        Self {
            id: None,
            name1: Some(String::from("test1234")),
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
async fn test_virtual_column() {
    let conn = setup_test_connection().await;

    if !TestItem15::table_exists(&conn).await.unwrap() {
        assert!(TestItem15::create_table(&conn).await.is_ok());
    } else {
        assert!(TestItem15::truncate_table(&conn).await.is_ok());
    }

    vec![TestItem15::test(), TestItem15::test2()].persist_all(&conn).await.unwrap();

    let results = TestItem15::query(&conn, TestItem15Column::NAME1.length().equals(7)).await;
    println!("{:?}", results);
    assert!(results.is_ok());
    assert_eq!(results.unwrap().len(), 1);

    assert!(TestItem15::drop_table(&conn).await.is_ok());
}