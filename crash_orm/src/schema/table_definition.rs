use postgres::types::Type;

use crate::{ColumnDefinition, DatabaseConnection};

/// Struct describing a table in the database
pub struct TableDefinition {
    pub(crate) old_name: String,
    pub(crate) name: String,
    pub(crate) columns: Vec<ColumnDefinition>,
}

impl TableDefinition {
    /// Creates a new table definition
    pub fn new(name: &str) -> Self {
        Self {
            old_name: String::new(),
            name: name.to_string(),
            columns: vec![],
        }
    }

    /// Load the table definition from the database
    pub async fn load_from_database(conn: &impl DatabaseConnection, name: &str) -> crate::Result<Self> {
        let rows = conn.query_many(
            "SELECT column_name, is_nullable, (SELECT oid FROM pg_catalog.pg_type pg_type WHERE pg_type.typname = c.udt_name) FROM information_schema.columns c WHERE table_schema = 'public' AND table_name = $1",
            &[&name.to_string()],
        ).await?;

        let columns = vec![];
        for column_row in rows {
            let name: String = column_row.get(0);
            let is_nullable: String = column_row.get(1);
            let sql_type_id: u32 = column_row.get(2);
            let sql_type = Type::from_oid(sql_type_id).unwrap();

            println!("Got column {} with type {:?} (nullable: {})", name, sql_type, is_nullable);
        }

        Ok(Self {
            old_name: name.to_string(),
            name: name.to_string(),
            columns,
        })
    }

    /// Rename the table
    pub fn rename(&mut self, new_name: &str) -> &mut TableDefinition {
        self.name = new_name.to_string();
        self
    }

    /// Edit a column
    pub fn edit_column<T: FnOnce(&mut ColumnDefinition)>(&mut self, name: &str, edit_fn: T) {
        let definition = self.columns.iter_mut()
            .find(|column| column.name == name).unwrap();
        edit_fn(definition);
    }

    /// Add a new column
    pub fn add_column(&mut self, column_definition: ColumnDefinition) {
        self.columns.push(column_definition);
    }

    /// Apply the changes to the database
    pub async fn apply(self, _conn: &impl DatabaseConnection) -> crate::Result<()> {
        Ok(())
    }
}