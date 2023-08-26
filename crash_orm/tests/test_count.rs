use crash_orm::{DatabaseConnection, Entity, EntityVec, NullQueryColumn, QueryEntity, Schema};
use crash_orm_derive::{Entity, Query, Schema};

pub async fn setup_test_connection() -> DatabaseConnection {
    DatabaseConnection::new("postgresql://crash_orm:postgres@localhost/crash_orm_test").await.unwrap()
}

#[derive(Entity, Debug, Schema, Query)]
pub struct TestItem6 {
    pub id: Option<u32>,
    pub name1: Option<String>,
    pub name2: Option<String>,
    pub number: Option<i32>,
}

impl TestItem6 {
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
async fn test_count() {
    let conn = setup_test_connection().await;

    if !TestItem6::table_exists(&conn).await.unwrap() {
        assert!(TestItem6::create_table(&conn).await.is_ok());
    } else {
        assert!(TestItem6::truncate_table(&conn).await.is_ok());
    }

    assert!(vec![TestItem6::test(), TestItem6::test2()].persist_all(&conn).await.is_ok());

    let result = TestItem6::count_query(&conn, TestItem6Column::NUMBER.is_null()).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1);

    let result = TestItem6::count(&conn).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 2);

    let result = TestItem6::count_column_query(
        &conn,
        TestItem6Column::NAME1,
        true,
        TestItem6Column::NAME1.is_not_null()
    ).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1);

    let result = TestItem6::count_column_query(
        &conn,
        TestItem6Column::NAME1,
        false,
        TestItem6Column::NAME1.is_not_null()
    ).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 2);

    let result = TestItem6::count_column(
        &conn,
        TestItem6Column::NUMBER,
        false,
    ).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1);

    assert!(TestItem6::drop_table(&conn).await.is_ok());
}