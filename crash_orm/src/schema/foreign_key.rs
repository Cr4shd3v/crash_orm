use postgres::Row;

#[derive(Eq, PartialEq, Clone)]
pub struct ForeignKey {
    pub(crate) name: Option<String>,
    pub(crate) src_field: String,
    pub(crate) target_table: String,
    pub(crate) target_field: String,
}

impl ForeignKey {
    pub(crate) fn from_row(row: &Row) -> Self {
        let constraint_name: String = row.get(0);
        let raw_def: String = row.get(1);
        let raw_def = raw_def.strip_prefix("FOREIGN_KEY (").unwrap();
        let parts = raw_def.splitn(2, ")").collect::<Vec<&str>>();
        let src_field = parts.first().unwrap().to_string();
        let raw_def = parts[1].strip_prefix(" REFERENCES ").unwrap();
        let parts = raw_def.split("(").collect::<Vec<&str>>();
        let target_table = parts.first().unwrap().to_string();
        let target_field = parts.last().unwrap().strip_suffix(")").unwrap().to_string();

        Self {
            name: Some(constraint_name),
            src_field,
            target_table,
            target_field,
        }
    }
}