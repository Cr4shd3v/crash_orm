use crash_orm::async_trait::async_trait;
use crash_orm::DatabaseConnection;

#[async_trait]
pub trait Migration<T: DatabaseConnection>: Send + Sync {
    async fn up(&self, conn: &T) -> crash_orm::Result<()>;

    async fn down(&self, conn: &T) -> crash_orm::Result<()>;

    fn get_name(&self) -> String;
}