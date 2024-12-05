use time::OffsetDateTime;

use crash_orm::prelude::*;
use crash_orm_test::{default_create_table, setup_test_connection};

#[derive(Entity, Debug, Schema)]
pub struct TestItemTime {
    id: u32,
    date: OffsetDateTime,
}

#[tokio::test]
pub async fn test_time() {
    let conn = setup_test_connection().await;
    default_create_table!(TestItemTime, conn);

    let date = OffsetDateTime::now_utc();

    TestItemTimeCreate {
        date,
    }.insert(&conn).await.unwrap();

    let result = TestItemTime::query()
        .condition(TestItemTimeColumn::DATE.equals(date))
        .fetch_single(&conn).await;
    assert!(result.is_ok());

    TestItemTime::drop_table(&conn).await.unwrap()
}