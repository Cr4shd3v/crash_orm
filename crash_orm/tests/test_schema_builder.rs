use tokio_postgres::types::Type;

use crash_orm::prelude::{ColumnDefinition, TableDefinition};
use crash_orm_test::setup_test_connection;

#[tokio::test]
pub async fn test_schema_builder() {
    let conn = setup_test_connection().await;

    TableDefinition::new("test_schema_builder")
        .add_column(ColumnDefinition::new("id", Type::OID, false)).unwrap()
        .add_column(ColumnDefinition::new("number", Type::INT4, false)).unwrap()
        .apply(&conn).await.unwrap();

    TableDefinition::load_from_database(&conn, "test_schema_builder").await.unwrap()
        .add_column(ColumnDefinition::new("test", Type::TEXT, true)).unwrap()
        .apply(&conn).await.unwrap();

    TableDefinition::load_from_database(&conn, "test_schema_builder").await.unwrap()
        .edit_column("test", |column| {
            column.set_primary(true);
        }).unwrap();

    TableDefinition::load_from_database(&conn, "test_schema_builder").await.unwrap()
        .drop_column("test").unwrap()
        .apply(&conn).await.unwrap();

    TableDefinition::load_from_database(&conn, "test_schema_builder").await.unwrap()
        .edit_column("number", |column| {
            column.change_type(Type::INT8)
                .rename("number2")
                .set_nullable(true)
                .set_default_value(Some("NULL".to_string()));
        }).unwrap()
        .apply(&conn).await.unwrap();

    TableDefinition::new("test_schema_builder_rel")
        .add_column(ColumnDefinition::new("id", Type::INT8, false).primary()).unwrap()
        .apply(&conn).await.unwrap();

    TableDefinition::load_from_database(&conn, "test_schema_builder").await.unwrap()
        .edit_column("number2", |column| {
            column.set_foreign_key("test_schema_builder_rel", "id");
        }).unwrap()
        .apply(&conn).await.unwrap();

    TableDefinition::drop_table(&conn, "test_schema_builder").await.unwrap();
    TableDefinition::drop_table(&conn, "test_schema_builder_rel").await.unwrap();
}