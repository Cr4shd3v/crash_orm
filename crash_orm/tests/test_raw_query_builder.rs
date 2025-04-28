use std::sync::Arc;
use crash_orm::prelude::*;
use crash_orm_test::{default_create_table, setup_test_connection};

#[derive(Entity, Schema, Debug)]
pub struct TestRawQueryBuilderItem {
    pub id: u32,
    pub text: String,
    pub number: f32,
}

#[derive(Entity, Schema, Debug)]
pub struct TestRawQueryBuilderRelItem {
    pub id: u32,
    pub rel: OneToOne<TestRawQueryBuilderItem, u32>,
}

#[tokio::test]
async fn test_raw_query_builder() {
    let conn = setup_test_connection().await;
    default_create_table!(TestRawQueryBuilderItem, conn);
    default_create_table!(TestRawQueryBuilderRelItem, conn);
    
    vec![TestRawQueryBuilderItemCreate {
        number: 1.5,
        text: "test123".to_string(),
    }, TestRawQueryBuilderItemCreate {
        number: 0.25,
        text: "foo bar".to_string(),
    }].insert_all(&conn).await.unwrap();
    
    vec![TestRawQueryBuilderRelItemCreate {
        rel: OneToOne::new(1),
    }].insert_all(&conn).await.unwrap();
    
    let mut query_builder = RawQueryBuilder::default();
    query_builder.add_from_entity::<TestRawQueryBuilderItem>("t1")
        .add_select("t1.text")
        .and_where("t1.number > _$i", vec![Arc::new(Box::new(1.0f32))]);
    
    let result = query_builder.query_many::<SingleResult<String>>(&conn).await.unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(*result[0], "test123");

    let mut query_builder = RawQueryBuilder::default();
    query_builder.add_from_entity::<TestRawQueryBuilderRelItem>("t1")
        .join(format!("{} t2 ON t2.id = t1.rel", TestRawQueryBuilderItem::TABLE_NAME))
        .add_select("t2.text");

    let result = query_builder.query_many::<SingleResult<String>>(&conn).await.unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(*result[0], "test123");
    
    TestRawQueryBuilderItem::drop_table(&conn).await.unwrap();
    TestRawQueryBuilderRelItem::drop_table(&conn).await.unwrap();
}
