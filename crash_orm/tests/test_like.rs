use rust_decimal::Decimal;
use tokio_postgres::NoTls;
use crash_orm::{DatabaseConnection, Entity, EntityVec, LikeQueryColumn, Schema};
use crash_orm_derive::{Entity, Schema};

pub async fn setup_test_connection() -> DatabaseConnection {
    DatabaseConnection::new("postgresql://crash_orm:postgres@localhost/crash_orm_test", NoTls).await.unwrap()
}

#[derive(Entity, Debug, Schema)]
pub struct TestItem11 {
    pub id: Option<u32>,
    pub name1: Option<String>,
    pub name2: Option<String>,
    pub number: Option<Decimal>,
}

impl TestItem11 {
    fn test() -> Self {
        Self {
            id: None,
            name1: Some(String::from("test1234")),
            name2: None,
            number: Some(Decimal::new(3200, 3)),
        }
    }

    fn test2() -> Self {
        Self {
            id: None,
            name1: Some(String::from("test123")),
            name2: Some(String::from("1234")),
            number: Some(Decimal::new(800, 3)),
        }
    }
}

#[tokio::test]
async fn test_like() {
    let conn = setup_test_connection().await;

    if !TestItem11::table_exists(&conn).await.unwrap() {
        assert!(TestItem11::create_table(&conn).await.is_ok());
    } else {
        assert!(TestItem11::truncate_table(&conn).await.is_ok());
    }

    vec![TestItem11::test(), TestItem11::test2()].persist_all(&conn).await.unwrap();

    let results = TestItem11::query().condition(TestItem11Column::NAME1.like(&String::from("test123%"))).execute(&conn).await;
    assert!(results.is_ok());
    assert_eq!(results.unwrap().len(), 2);

    let results = TestItem11::query().condition(TestItem11Column::NAME1.not_like(&String::from("test1234%"))).execute(&conn).await;
    assert!(results.is_ok());
    assert_eq!(results.unwrap().len(), 1);

    assert!(TestItem11::drop_table(&conn).await.is_ok());
}