use crate::DatabaseConnection;

pub trait Schema {
    fn create_table(connection: &DatabaseConnection) -> crate::Result<()>;

    fn drop_table(connection: &DatabaseConnection) -> crate::Result<()>;

    fn truncate_table(connection: &DatabaseConnection) -> crate::Result<()>;
}