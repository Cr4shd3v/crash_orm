use tokio_postgres::NoTls;
use crash_orm::{DatabaseConnection, Entity, EntityVec, ManyToOne, OneToMany, Schema};
use crash_orm_derive::{Entity, Schema};

pub async fn setup_test_connection() -> DatabaseConnection {
    DatabaseConnection::new("postgresql://crash_orm:postgres@localhost/crash_orm_test", NoTls).await.unwrap()
}

#[derive(Entity, Debug, Schema)]
pub struct TestItem21 {
    pub id: Option<u32>,
    pub name1: Option<String>,
    pub active: bool,
    pub other: Option<ManyToOne<TestItem22>>,
}

#[derive(Entity, Debug, Schema)]
pub struct TestItem22 {
    pub id: Option<u32>,
    pub name1: Option<String>,
    pub active: bool,
    #[mapped_by("other")]
    pub test_items_21: OneToMany<TestItem21>,
}

impl TestItem21 {
    fn test() -> Self {
        Self {
            id: None,
            name1: Some(String::from("test123")),
            active: false,
            other: None,
        }
    }

    fn test2() -> Self {
        Self {
            id: None,
            name1: Some(String::from("test321")),
            active: true,
            other: Some(ManyToOne::new(1)),
        }
    }
}

impl TestItem22 {
    fn test() -> Self {
        Self {
            id: None,
            name1: Some(String::from("Test1234")),
            active: false,
            test_items_21: OneToMany::new(),
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

    let mut target_item = TestItem22::test();
    target_item.persist(&conn).await.unwrap();
    let mut test_item = TestItem21::test();
    test_item.other = Some(ManyToOne::from(&target_item).unwrap());
    vec![test_item, TestItem21::test2()].persist_all(&conn).await.unwrap();

    let results = TestItem21::query()
        .execute(&conn).await;
    assert!(results.is_ok());
    let results = results.unwrap();
    assert_eq!(results.len(), 2);
    assert_eq!(results[0].other.as_ref().unwrap().get(&conn).await.unwrap().name1, Some(String::from("Test1234")));
    assert_eq!(results[1].other.as_ref().unwrap().get(&conn).await.unwrap().name1, Some(String::from("Test1234")));

    let results = target_item.get_test_items_21(&conn).await;
    assert!(results.is_ok());
    assert_eq!(results.unwrap().len(), 2);

    assert!(TestItem21::drop_table(&conn).await.is_ok());
    assert!(TestItem22::drop_table(&conn).await.is_ok());
}