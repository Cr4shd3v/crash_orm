use tokio_postgres::types::ToSql;
use crate::{Entity, EntityColumn, QueryCondition};

pub trait LikeQueryColumn<T: ToSql, U: Entity<U> + Send + 'static> {
    fn like(&self, like: String) -> QueryCondition<U>;

    fn not_like(&self, like: String) -> QueryCondition<U>;
}

macro_rules! impl_like_entity_column {
    ($column_type:ty) => {
        impl<U: Entity<U> + Send + 'static> LikeQueryColumn<String, U> for EntityColumn<'_, $column_type, U> {
            fn like(&self, like: String) -> QueryCondition<U> {
                QueryCondition::Like(self.get_name(), like)
            }

            fn not_like(&self, like: String) -> QueryCondition<U> {
                QueryCondition::NotLike(self.get_name(), like)
            }
        }
    };
}

impl_like_entity_column!(String);
impl_like_entity_column!(Option<String>);