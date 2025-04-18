use crash_orm::prelude::*;
use crash_orm_test::{default_create_table, setup_test_connection};

#[derive(Entity, Schema, Debug)]
pub struct TestItemJson {
    pub id: u32,
    pub data: TypedJson<Vec<String>>,
}

#[tokio::test]
pub async fn test_json() {
    let conn = setup_test_connection().await;
    default_create_table!(TestItemJson, conn);
    
    let example_json = vec!["test1".to_string(), "test2".to_string()];
    TestItemJsonCreate {
        data: example_json.clone().into(),
    }.insert(&conn).await.unwrap();
    
    let results = TestItemJson::query()
        .condition(TestItemJsonColumn::DATA.equals(TypedJson(example_json.clone())))
        .fetch_single(&conn).await.unwrap();
    assert!(results.is_some());
    let results = results.unwrap();
    assert_eq!(*results.data, example_json);

    TestItemJson::drop_table(&conn).await.unwrap();
}