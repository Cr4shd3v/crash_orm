use geo_types::Point;

use crash_orm::{Entity, EqualQueryColumn, Schema};
use crash_orm_derive::{Entity, Schema};
use crash_orm_test::{default_create_table, setup_test_connection};

#[derive(Entity, Debug, Schema)]
pub struct TestItemGeoTypes {
    id: Option<u32>,
    point: Point,
}

#[tokio::test]
pub async fn test_geo_types() {
    let conn = setup_test_connection().await;
    default_create_table!(TestItemGeoTypes, conn);

    let point = Point::new(2.0, 2.0);
    let entity = TestItemGeoTypes {
        id: None,
        point,
    };
    entity.insert_get_id(&conn).await.unwrap();
    let result = TestItemGeoTypes::query()
        .condition(TestItemGeoTypesColumn::POINT.equals(point))
        .fetch_single(&conn).await;
    assert!(result.is_ok());

    TestItemGeoTypes::drop_table(&conn).await.unwrap();
}