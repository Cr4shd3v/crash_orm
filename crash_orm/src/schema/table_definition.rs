use postgres::types::Type;

use crate::{ColumnDefinition, DatabaseConnection};

/// Struct describing a table in the database
pub struct TableDefinition {
    pub(crate) old_name: Option<String>,
    pub(crate) name: String,
    pub(crate) columns: Vec<ColumnDefinition>,
    dropped_columns: Vec<String>,
}

impl TableDefinition {
    /// Creates a new table definition
    pub fn new(name: &str) -> Self {
        Self {
            old_name: None,
            name: name.to_string(),
            columns: vec![],
            dropped_columns: vec![],
        }
    }

    /// Load the table definition from the database
    pub async fn load_from_database(conn: &impl DatabaseConnection, name: &str) -> crate::Result<Self> {
        let rows = conn.query_many(
            "SELECT column_name, is_nullable, (SELECT oid FROM pg_catalog.pg_type pg_type WHERE pg_type.typname = c.udt_name) FROM information_schema.columns c WHERE table_schema = 'public' AND table_name = $1",
            &[&name.to_string()],
        ).await?;

        let mut columns = vec![];
        for column_row in rows {
            let name: String = column_row.get(0);
            let is_nullable: String = column_row.get(1);
            let sql_type_id: u32 = column_row.get(2);
            let sql_type = Type::from_oid(sql_type_id).unwrap();

            columns.push(ColumnDefinition::from_database(name, sql_type, is_nullable == "YES"));
        }

        Ok(Self {
            old_name: Some(name.to_string()),
            name: name.to_string(),
            columns,
            dropped_columns: vec![],
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
    pub fn add_column(&mut self, column_definition: ColumnDefinition) -> crate::Result<()> {
        if self.columns.iter().find(|column| column.name == column_definition.name).is_some() {
            return Err(crate::Error::String(String::from("Can't add another column with the same name")));
        }

        self.columns.push(column_definition);

        Ok(())
    }

    /// Drop a column
    pub fn drop_column(&mut self, name: &str) -> crate::Result<()> {
        self.dropped_columns.push(name.to_string());
        let Some((index, _)) = self.columns.iter().enumerate().find(|(_, v)| v.name == name && v.old_name.is_some()) else {
            return Err(crate::Error::String(format!("Tried to remove non existing column {}", name)));
        };
        self.columns.remove(index);

        Ok(())
    }

    /// Apply the changes to the database
    pub async fn apply(self, conn: &impl DatabaseConnection) -> crate::Result<()> {
        if let Some(ref old_name) = self.old_name {
            if &**old_name != &*self.name {
                conn.execute_query(&*format!("ALTER TABLE {} RENAME TO {}", old_name, self.name), &[]).await?;
            }

            let mut alters = vec![];

            for dropped_column in self.dropped_columns {
                alters.push(format!("DROP COLUMN {}", dropped_column));
            }

            for column in self.columns {
                if column.old_name.is_none() {
                    let mut string = format!("ADD COLUMN {} {}", column.name, column.sql_type.name());
                    if !column.nullable {
                        string.push_str(" NOT NULL");
                    }
                    alters.push(string);
                } else {
                    let old_name = column.old_name.unwrap();
                    if old_name != column.name {
                        alters.push(format!("RENAME COLUMN {} TO {}", old_name, column.name));
                    }

                    let old_sql_type = column.old_sql_type.unwrap();
                    if old_sql_type != column.sql_type {
                        alters.push(format!("ALTER COLUMN {} TYPE {}", column.name, column.sql_type.name()));
                    }

                    let old_nullable = column.old_nullable.unwrap();
                    if old_nullable != column.nullable {
                        if !column.nullable {
                            alters.push(format!("ALTER COLUMN {} DROP NOT NULL", column.name));
                        } else {
                            alters.push(format!("ALTER COLUMN {} SET NOT NULL", column.name));
                        }
                    }
                }
            }

            let query = format!("ALTER TABLE {} {}", self.name, alters.join(","));

            conn.execute_query(&*query, &[]).await?;
        } else {
            // New table
        }

        Ok(())
    }
}