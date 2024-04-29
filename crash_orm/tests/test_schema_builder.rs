use tokio_postgres::types::Type;

use crash_orm::{ColumnDefinition, TableDefinition};
use crash_orm_test::setup_test_connection;

#[tokio::test]
pub async fn test_schema_builder() {
    let conn = setup_test_connection().await;

    // let mut new_table = TableDefinition::new("test_schema_builder");
    // new_table.add_column(ColumnDefinition::new("test", Type::TEXT, false));
    //
    // new_table.apply(&conn).await.unwrap();

    let mut table = TableDefinition::load_from_database(&conn, "test_entity").await.unwrap();

    table.add_column(ColumnDefinition::new("test", Type::TEXT, true)).unwrap();

    table.apply(&conn).await.unwrap()
}