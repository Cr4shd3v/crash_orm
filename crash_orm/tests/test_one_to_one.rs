use crash_orm::entity::CreateEntity;
use crash_orm::prelude::{Entity, OneToOne, OneToOneRef, Schema};
use crash_orm_test::setup_test_connection;

#[derive(Entity, Debug, Schema)]
pub struct TestItem19 {
    pub id: u32,
    pub name1: Option<String>,
    pub active: bool,
    pub other: Option<OneToOne<TestItem20, u32>>,
}

#[derive(Entity, Debug, Schema)]
pub struct TestItem20 {
    pub id: u32,
    pub name1: Option<String>,
    pub active: bool,
    #[mapped_by("other")]
    pub other: OneToOneRef<TestItem19, u32>,
}

impl TestItem19Create {
    fn test2() -> Self {
        Self {
            name1: Some(String::from("test123")),
            active: true,
            other: Some(OneToOne::new(1)),
        }
    }
}

impl TestItem20Create {
    fn test() -> Self {
        Self {
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

    let test_item_20 = TestItem20Create::test().insert(&conn).await.unwrap();
    TestItem19Create::test2().insert(&conn).await.unwrap();

    let result = test_item_20.get_other(&conn).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().unwrap().name1, Some(String::from("test123")));

    let results = TestItem19::query().fetch(&conn).await;
    assert!(results.is_ok());
    let results = results.unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(
        results[0].get_other(&conn).await.unwrap().unwrap().name1,
        Some(String::from("Test1234"))
    );

    assert!(TestItem19::drop_table(&conn).await.is_ok());
    assert!(TestItem20::drop_table(&conn).await.is_ok());
}
