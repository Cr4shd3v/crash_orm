use tokio_postgres::types::Type;

use crash_orm::{ColumnDefinition, TableDefinition};
use crash_orm_test::setup_test_connection;

#[tokio::test]
pub async fn test_schema_builder() {
    let conn = setup_test_connection().await;

    let mut new_table = TableDefinition::new("test_schema_builder");
    new_table.add_column(ColumnDefinition::new("id", Type::OID, false)).unwrap();
    new_table.add_column(ColumnDefinition::new("number", Type::INT4, false)).unwrap();

    new_table.apply(&conn).await.unwrap();

    let mut table = TableDefinition::load_from_database(&conn, "test_schema_builder").await.unwrap();
    table.add_column(ColumnDefinition::new("test", Type::TEXT, true)).unwrap();
    table.apply(&conn).await.unwrap();

    let mut table = TableDefinition::load_from_database(&conn, "test_schema_builder").await.unwrap();
    table.drop_column("test").unwrap();
    table.apply(&conn).await.unwrap();

    let mut table = TableDefinition::load_from_database(&conn, "test_schema_builder").await.unwrap();
    table.edit_column("number", |c| {
        c.change_type(Type::INT8);
        c.rename("number2");
        c.set_nullable(true);
    });
    table.apply(&conn).await.unwrap();

    TableDefinition::drop_table(&conn, "test_schema_builder").await.unwrap();
}