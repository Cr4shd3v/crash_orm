use crash_orm::{DatabaseConnection, Entity, EntityVec, InQueryColumn, Schema};
use crash_orm_derive::{Entity, Schema};

pub async fn setup_test_connection() -> DatabaseConnection {
    DatabaseConnection::new("postgresql://crash_orm:postgres@localhost/crash_orm_test").await.unwrap()
}

#[derive(Entity, Debug, Schema)]
pub struct TestItem14 {
    pub id: Option<u32>,
    pub name1: Option<String>,
    pub active: bool,
    pub number: Option<i32>,
}

impl TestItem14 {
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
async fn test_in() {
    let conn = setup_test_connection().await;

    if !TestItem14::table_exists(&conn).await.unwrap() {
        assert!(TestItem14::create_table(&conn).await.is_ok());
    } else {
        assert!(TestItem14::truncate_table(&conn).await.is_ok());
    }

    vec![TestItem14::test(), TestItem14::test2()].persist_all(&conn).await.unwrap();

    let results = TestItem14::query().condition(TestItem14Column::NUMBER.in_vec(vec![&439, &440])).execute(&conn).await;
    println!("{:?}", results);
    assert!(results.is_ok());
    assert_eq!(results.unwrap().len(), 1);

    let results = TestItem14::query().condition(TestItem14Column::NUMBER.not_in_vec(vec![&439, &440])).execute(&conn).await;
    assert!(results.is_ok());
    assert_eq!(results.unwrap().len(), 1);

    let results = TestItem14::query().condition(TestItem14Column::NAME1.in_vec(vec![&String::from("test12"), &String::from("test1234")])).execute(&conn).await;
    assert!(results.is_ok());
    assert_eq!(results.unwrap().len(), 1);

    assert!(TestItem14::drop_table(&conn).await.is_ok());
}