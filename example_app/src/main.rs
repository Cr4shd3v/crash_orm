use crash_orm::crash_orm_derive::entity;
use crash_orm::{DatabaseConnection, Entity};

#[tokio::main]
async fn main() {
    let conn = DatabaseConnection::new("postgresql://crash_orm:postgres@localhost").await.unwrap();

    let row = conn.query_one("SELECT $1::TEXT;", &[&"hello world"]).await.unwrap();
    let value: &str = row.get(0);
    println!("{}", value);
    let mut item = TestItem::new(String::from("test"));
    item.persist(&conn).await.unwrap();
    // item.remove(&conn).await.unwrap();
    println!("{:?}", item.id);
}

#[entity]
pub struct TestItem {
    pub name: String,
}

impl TestItem {
    pub fn new(name: String) -> Self {
        Self {
            name,
            id: None,
        }
    }
}