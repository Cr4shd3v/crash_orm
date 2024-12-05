use eui48::MacAddress;

use crash_orm::prelude::*;
use crash_orm_test::{default_create_table, setup_test_connection};

#[derive(Entity, Debug, Schema)]
pub struct TestItemEuI48 {
    id: Option<u32>,
    mac: MacAddress,
}

#[tokio::test]
async fn test_eui48() {
    let conn = setup_test_connection().await;
    default_create_table!(TestItemEuI48, conn);

    let mac = MacAddress::broadcast();
    let mut entity = TestItemEuI48 {
        id: None,
        mac,
    };
    entity.insert(&conn).await.unwrap();

    let item = TestItemEuI48::query()
        .condition(TestItemEuI48Column::MAC.equals(mac))
        .fetch_single(&conn).await;
    assert!(item.is_ok());

    TestItemEuI48::drop_table(&conn).await.unwrap();
}