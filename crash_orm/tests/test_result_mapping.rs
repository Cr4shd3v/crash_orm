use crash_orm::prelude::*;
use crash_orm_test::{default_create_table, setup_test_connection};

#[derive(Entity, Debug, Schema)]
struct TestResultMapping {
    id: Option<u32>,
    number: i32,
    test: String,
    test2: String,
}

impl TestResultMapping {
    fn test_items() -> Vec<TestResultMapping> {
        vec![
            TestResultMapping {
                id: None,
                number: 1,
                test: "test".to_string(),
                test2: "test2".to_string(),
            },
            TestResultMapping {
                id: None,
                number: 2,
                test: "tes31t".to_string(),
                test2: "test42".to_string(),
            },
            TestResultMapping {
                id: None,
                number: 3,
                test: "te5st".to_string(),
                test2: "tes234t2".to_string(),
            },
        ]
    }
}

#[derive(ResultMapping)]
struct TestMapping {
    count: i64,
    sum: i64,
}

#[tokio::test]
async fn test_result_mapping() {
    let conn = setup_test_connection().await;
    
    default_create_table!(TestResultMapping, conn);
    
    TestResultMapping::test_items().insert_all(&conn).await.unwrap();
    
    let result = TestResultMapping::select_query::<TestMapping>(&[
        &TestResultMappingColumn::TEST.count_column(true),
        &TestResultMappingColumn::NUMBER.sum(true),
    ]).fetch_single(&conn).await.unwrap();
    assert_eq!(result.count, 3);
    assert_eq!(result.sum, 6);
    
    TestResultMapping::drop_table(&conn).await.unwrap();
}