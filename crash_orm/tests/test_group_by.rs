use crash_orm::prelude::*;
use crash_orm_test::setup_test_connection;
use tokio_postgres::Row;
use uuid::Uuid;

#[derive(Entity, Debug, Schema)]
pub struct TestItemGroupBy {
    id: Uuid,
    number: i32,
}

impl TestItemGroupByCreate {
    fn test_items() -> Vec<Self> {
        vec![
            Self {
                number: 1,
            },
            Self {
                number: 1,
            },
            Self {
                number: 2,
            },
        ]
    }
}

#[tokio::test]
async fn test_group_by() {
    let conn = setup_test_connection().await;

    if !TestItemGroupBy::table_exists(&conn).await.unwrap() {
        assert!(TestItemGroupBy::create_table(&conn).await.is_ok());
    } else {
        assert!(TestItemGroupBy::truncate_table(&conn).await.is_ok());
    }

    TestItemGroupByCreate::test_items().insert_all(&conn).await.unwrap();

    let results = TestItemGroupBy::select_query::<Row>(&[&TestItemGroupByColumn::ID.count_column(false)])
        .add_group_by(&TestItemGroupByColumn::NUMBER)
        .add_order(&TestItemGroupByColumn::NUMBER, OrderDirection::ASC)
        .fetch(&conn).await.unwrap();

    assert_eq!(results.len(), 2);
    let results = results.into_iter().map(|v| v.get::<_, i64>(0)).collect::<Vec<i64>>();
    assert_eq!(results[0], 2);
    assert_eq!(results[1], 1);

    assert!(TestItemGroupBy::drop_table(&conn).await.is_ok());
}