use crate::entity::Entity;
use postgres::Row;

pub trait ResultMapping {
    fn from_row(row: Row) -> Self where Self: Sized;
}

impl ResultMapping for Row {
    fn from_row(row: Row) -> Self
    where
        Self: Sized
    {
        row
    }
}

impl<T: Entity> ResultMapping for T {
    fn from_row(row: Row) -> Self
    where
        Self: Sized
    {
        T::load_from_row(&row)
    }
}