use tokio_postgres::Row;

pub trait Entity {
    type Output;

    fn load_from_row(row: Row) -> Self::Output;

    fn get_select_query(&self) -> String;

    fn get_insert_stmt(&self) -> String;
}