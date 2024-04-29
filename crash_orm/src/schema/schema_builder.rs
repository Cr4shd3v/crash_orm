use crate::{DatabaseConnection, Entity, PrimaryKey};

/// Struct used to modify a table for migrations
pub struct SchemaBuilder {
    table_name: String,
}

impl SchemaBuilder {
    /// Load table schema from the database
    pub fn table(table_name: String) -> Self {
        Self {
            table_name,
        }
    }

    pub async fn create(conn: &impl DatabaseConnection) {

    }

    pub async fn edit(conn: &impl DatabaseConnection) {

    }
}

/// Helper trait to be able to create a [SchemaBuilder]
pub trait SchemaBuilderFromEntity<P: PrimaryKey> {
    /// Load the schema from the provided [Entity]
    fn from_entity<T: Entity<T, P>>() -> SchemaBuilder;
}

impl<P: PrimaryKey> SchemaBuilderFromEntity<P> for SchemaBuilder {
    fn from_entity<T: Entity<T, P>>() -> SchemaBuilder {
        SchemaBuilder::table(T::TABLE_NAME.to_string())
    }
}