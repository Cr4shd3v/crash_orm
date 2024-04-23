use crash_orm::{Entity, OneToOne, OneToOneRef, Schema};
use crash_orm_derive::{Entity, Schema};
use crash_orm_test::setup_test_connection;

#[derive(Entity, Debug, Schema)]
pub struct TestItem19 {
    pub id: Option<u32>,
    pub name1: Option<String>,
    pub active: bool,
    pub other: Option<OneToOne<TestItem20, u32>>,
}

#[derive(Entity, Debug, Schema)]
pub struct TestItem20 {
    pub id: Option<u32>,
    pub name1: Option<String>,
    pub active: bool,
    #[mapped_by("other")]
    pub other: OneToOneRef<TestItem19, u32>,
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
            other: OneToOneRef::new(),
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

    let mut test_item_20 = TestItem20::test();
    test_item_20.persist(&conn).await.unwrap();
    TestItem19::test2().persist(&conn).await.unwrap();

    let result = test_item_20.get_other(&conn).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().name1, Some(String::from("test123")));

    let results = TestItem19::query().execute(&conn).await;
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
