use postgres::types::Type;

/// Struct describing a column in a table
pub struct ColumnDefinition {
    pub(crate) old_name: Option<String>,
    pub(crate) name: String,
    pub(crate) old_sql_type: Option<Type>,
    pub(crate) sql_type: Type,
    pub(crate) old_nullable: Option<bool>,
    pub(crate) nullable: bool,
    pub(crate) primary_key: bool,
    pub(crate) old_default_value: Option<Option<String>>,
    pub(crate) default_value: Option<String>,
}

impl ColumnDefinition {
    /// Creates a new column definition
    pub fn new(name: &'static str, sql_type: Type, nullable: bool) -> Self {
        Self {
            old_name: None,
            name: name.to_string(),
            old_sql_type: None,
            sql_type,
            old_nullable: None,
            nullable,
            primary_key: false,
            old_default_value: None,
            default_value: None,
        }
    }

    /// Creates a column definition from the database
    pub(crate) fn from_database(name: String, sql_type: Type, nullable: bool, primary_key: bool, default_value: Option<String>) -> Self {
        Self {
            old_name: Some(name.to_string()),
            name,
            old_sql_type: Some(sql_type.clone()),
            sql_type,
            old_nullable: Some(nullable),
            nullable,
            primary_key,
            old_default_value: Some(default_value.clone()),
            default_value,
        }
    }

    /// Rename the column
    pub fn rename(&mut self, new_name: &'static str) -> &mut ColumnDefinition {
        self.name = new_name.to_string();
        self
    }

    /// Change null constraint
    pub fn set_nullable(&mut self, nullable: bool) -> &mut ColumnDefinition {
        self.nullable = nullable;
        self
    }

    /// Change type of column
    pub fn change_type(&mut self, sql_type: Type) -> &mut ColumnDefinition {
        self.sql_type = sql_type;
        self
    }

    /// Change primary key constraint
    pub fn set_primary(&mut self, primary: bool) -> &mut ColumnDefinition {
        self.primary_key = primary;
        self
    }

    /// Change the default value
    pub fn set_default_value(&mut self, default_value: Option<String>) -> &mut ColumnDefinition {
        self.default_value = default_value;
        self
    }
}