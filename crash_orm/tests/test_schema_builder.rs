use tokio_postgres::types::Type;

use crash_orm::prelude::{ColumnDefinition, TableDefinition};
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
    table.edit_column("test", |column| {
        column.set_primary(true);
    });

    let mut table = TableDefinition::load_from_database(&conn, "test_schema_builder").await.unwrap();
    table.drop_column("test").unwrap();
    table.apply(&conn).await.unwrap();

    let mut table = TableDefinition::load_from_database(&conn, "test_schema_builder").await.unwrap();
    table.edit_column("number", |column| {
        column.change_type(Type::INT8)
            .rename("number2")
            .set_nullable(true)
            .set_default_value(Some("NULL".to_string()));
    });
    table.apply(&conn).await.unwrap();

    let mut rel_table = TableDefinition::new("test_schema_builder_rel");
    rel_table.add_column(ColumnDefinition::new("id", Type::INT8, false).primary()).unwrap();
    rel_table.apply(&conn).await.unwrap();

    let mut table = TableDefinition::load_from_database(&conn, "test_schema_builder").await.unwrap();
    table.edit_column("number2", |column| {
        column.set_foreign_key("test_schema_builder_rel", "id");
    });
    table.apply(&conn).await.unwrap();

    TableDefinition::drop_table(&conn, "test_schema_builder").await.unwrap();
    TableDefinition::drop_table(&conn, "test_schema_builder_rel").await.unwrap();
}