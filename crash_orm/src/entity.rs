use tokio_postgres::Row;

pub trait Entity {
    type Output;

    fn load_from_row(row: Row) -> Self::Output;
}