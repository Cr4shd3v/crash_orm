use tokio_postgres::types::ToSql;
use crate::{Entity, EntityColumn, QueryCondition};

pub trait NullQueryColumn<T: ToSql, U: Entity<U> + Send + 'static> {
    fn is_null(&self) -> QueryCondition<U>;

    fn is_not_null(&self) -> QueryCondition<U>;
}

impl<T: ToSql, U: Entity<U> + Send + 'static> NullQueryColumn<T, U> for EntityColumn<Option<T>, U>  {
    fn is_null(&self) -> QueryCondition<U> {
        QueryCondition::IsNull(self.name.to_string())
    }

    fn is_not_null(&self) -> QueryCondition<U> {
        QueryCondition::IsNotNull(self.name.to_string())
    }
}

pub trait EqualQueryColumn<T: ToSql, U: Entity<U> + Send + 'static> {
    fn equals(&self, other: T) -> QueryCondition<U>;

    fn not_equals(&self, other: T) -> QueryCondition<U>;
}

macro_rules! impl_equal_entity_column {
    ($column_type:ty) => {
        impl<T: Entity<T> + Send + 'static> EqualQueryColumn<$column_type, T> for EntityColumn<$column_type, T> {
            fn equals(&self, other: $column_type) -> QueryCondition<T> {
                QueryCondition::Equals(self.name.to_string(), Box::new(other))
            }

            fn not_equals(&self, other: $column_type) -> QueryCondition<T> {
                QueryCondition::NotEquals(self.name.to_string(), Box::new(other))
            }
        }

        impl<T: Entity<T> + Send + 'static> EqualQueryColumn<$column_type, T> for EntityColumn<Option<$column_type>, T> {
            fn equals(&self, other: $column_type) -> QueryCondition<T> {
                QueryCondition::Equals(self.name.to_string(), Box::new(other))
            }

            fn not_equals(&self, other: $column_type) -> QueryCondition<T> {
                QueryCondition::NotEquals(self.name.to_string(), Box::new(other))
            }
        }
    };
}

impl_equal_entity_column!(bool);
impl_equal_entity_column!(i8);
impl_equal_entity_column!(i16);
impl_equal_entity_column!(i32);
impl_equal_entity_column!(i64);
impl_equal_entity_column!(u32);
impl_equal_entity_column!(f32);
impl_equal_entity_column!(f64);
impl_equal_entity_column!(String);