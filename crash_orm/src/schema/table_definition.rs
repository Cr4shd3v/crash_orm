use std::collections::HashSet;
use postgres::types::Type;

use crate::{ColumnDefinition, DatabaseConnection};

/// Struct describing a table in the database
pub struct TableDefinition {
    old_name: Option<String>,
    name: String,
    columns: Vec<ColumnDefinition>,
    dropped_columns: Vec<String>,
    old_primary_keys: Option<Vec<String>>,
}

impl TableDefinition {
    /// Creates a new table definition
    pub fn new(name: &str) -> Self {
        Self {
            old_name: None,
            name: name.to_string(),
            columns: vec![],
            dropped_columns: vec![],
            old_primary_keys: None,
        }
    }

    /// Load the table definition from the database
    pub async fn load_from_database(conn: &impl DatabaseConnection, name: &str) -> crate::Result<Self> {
        let primary_key_query = format!("SELECT
  pg_attribute.attname
FROM pg_index, pg_class, pg_attribute, pg_namespace
WHERE
  pg_class.oid = '{}'::regclass AND
  indrelid = pg_class.oid AND
  nspname = 'public' AND
  pg_class.relnamespace = pg_namespace.oid AND
  pg_attribute.attrelid = pg_class.oid AND
  pg_attribute.attnum = any(pg_index.indkey)
 AND indisprimary", name);
        let primary_key_rows = conn.query_many(&*primary_key_query, &[]).await?;
        let primary_keys = primary_key_rows.iter()
            .map(|row| row.get::<usize, String>(0))
            .collect::<Vec<String>>();

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
            let is_primary = primary_keys.contains(&name);

            columns.push(ColumnDefinition::from_database(name, sql_type, is_nullable == "YES", is_primary));
        }

        Ok(Self {
            old_name: Some(name.to_string()),
            name: name.to_string(),
            columns,
            dropped_columns: vec![],
            old_primary_keys: Some(primary_keys),
        })
    }

    /// Drops the table with provided `table_name`
    pub async fn drop_table(conn: &impl DatabaseConnection, table_name: &str) -> crate::Result<()> {
        conn.execute_query(&*format!("DROP TABLE IF EXISTS {} CASCADE", table_name), &[]).await?;
        Ok(())
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

            let old_primary_keys = self.old_primary_keys.unwrap();
            let mut primary_keys = vec![];
            let mut primary_keys_dropped = false;
            let mut alters = vec![];

            for dropped_column in self.dropped_columns {
                if old_primary_keys.contains(&dropped_column) && !primary_keys_dropped {
                    alters.push(format!("DROP CONSTRAINT {}_pkey", self.name));
                    primary_keys_dropped = true;
                }

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
                        conn.execute_query(&*format!("ALTER TABLE {} RENAME COLUMN {} TO {}", self.name, old_name, column.name), &[]).await?;
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

                    if column.primary_key {
                        primary_keys.push(column.name);
                    }
                }
            }

            if old_primary_keys.iter().collect::<HashSet<&String>>() != primary_keys.iter().collect::<HashSet<&String>>() {
                if !primary_keys_dropped {
                    alters.push(format!("DROP CONSTRAINT {}_pkey", self.name));
                }

                alters.push(format!("ADD PRIMARY KEY ({})", primary_keys.join(",")))
            }

            let query = format!("ALTER TABLE {} {}", self.name, alters.join(","));

            conn.execute_query(&*query, &[]).await?;
        } else {
            let mut columns = vec![];
            let mut primary_columns = vec![];

            for column in self.columns {
                let mut string = format!("{} {} ", column.name, column.sql_type.name());

                if column.nullable {
                    string.push_str("NULL");
                } else {
                    string.push_str("NOT NULL");
                }

                if column.primary_key {
                    primary_columns.push(column.name);
                }

                columns.push(string);
            }

            if !primary_columns.is_empty() {
                columns.push(format!("PRIMARY KEY ({})", primary_columns.join(",")));
            }

            let query = format!("CREATE TABLE public.{}({})", self.name, columns.join(","));
            conn.execute_query(&*query, &[]).await?;
        }

        Ok(())
    }
}