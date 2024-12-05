use crash_orm::prelude::{CreateEntity, Entity, ManyToOne, OneToMany, Schema};
use crash_orm_test::setup_test_connection;

#[derive(Entity, Debug, Schema)]
pub struct TestItem21 {
    pub id: u32,
    pub name1: Option<String>,
    pub active: bool,
    pub other: Option<ManyToOne<TestItem22, u32>>,
}

#[derive(Entity, Debug, Schema)]
pub struct TestItem22 {
    pub id: u32,
    pub name1: Option<String>,
    pub active: bool,
    #[mapped_by("other")]
    pub test_items_21: OneToMany<TestItem21, u32>,
}

impl TestItem21Create {
    fn test() -> Self {
        Self {
            name1: Some(String::from("test123")),
            active: false,
            other: None,
        }
    }

    fn test2() -> Self {
        Self {
            name1: Some(String::from("test321")),
            active: true,
            other: Some(ManyToOne::new(1)),
        }
    }
}

impl TestItem22Create {
    fn test() -> Self {
        Self {
            name1: Some(String::from("Test1234")),
            active: false,
        }
    }
}

#[tokio::test]
async fn test_many_to_one() {
    let conn = setup_test_connection().await;

    if TestItem22::table_exists(&conn).await.unwrap() {
        assert!(TestItem22::drop_table(&conn).await.is_ok());
    }

    assert!(TestItem22::create_table(&conn).await.is_ok());

    if TestItem21::table_exists(&conn).await.unwrap() {
        assert!(TestItem21::drop_table(&conn).await.is_ok());
    }

    assert!(TestItem21::create_table(&conn).await.is_ok());

    let target_item = TestItem22Create::test().insert(&conn).await.unwrap();
    let mut test_item = TestItem21Create::test().insert(&conn).await.unwrap();
    test_item.set_other(Some(&target_item)).unwrap();
    test_item.update(&conn).await.unwrap();
    TestItem21Create::test2().insert(&conn).await.unwrap();

    let results = TestItem21::query().fetch(&conn).await;
    assert!(results.is_ok());
    let results = results.unwrap();
    assert_eq!(results.len(), 2);
    assert_eq!(
        results[0].get_other(&conn).await.unwrap().unwrap().name1,
        Some(String::from("Test1234"))
    );
    assert_eq!(
        results[1].get_other(&conn).await.unwrap().unwrap().name1,
        Some(String::from("Test1234"))
    );

    let results = target_item.get_test_items_21(&conn).await;
    assert!(results.is_ok());
    assert_eq!(results.unwrap().len(), 2);

    assert!(TestItem21::drop_table(&conn).await.is_ok());
    assert!(TestItem22::drop_table(&conn).await.is_ok());
}
