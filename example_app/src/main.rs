use crash_orm::crash_orm_derive::entity;
use crash_orm::DatabaseConnection;

#[tokio::main]
async fn main() {
    let conn = DatabaseConnection::new("postgresql://crash_orm:postgres@localhost").await.unwrap();

    let row = conn.query_one("SELECT $1::TEXT;", &[&"hello world"]).await.unwrap();
    let value: &str = row.get(0);
    println!("{}", value);
}

#[entity]
pub struct TestItem {
    pub name: String,
}