use crash_orm::crash_orm_derive::Entity;
use crash_orm::{DatabaseConnection, Entity};

#[tokio::main]
async fn main() {
    let conn = DatabaseConnection::new("postgresql://crash_orm:postgres@localhost").await.unwrap();
    // let mut item = TestItem::new(String::from("test"));
    // item.persist(&conn).await.unwrap();
    // item.remove(&conn).await.unwrap();
    let mut item = TestItem::get_by_id(&conn, 10).await.unwrap();
    item.name = String::from("test123");
    item.persist(&conn).await.unwrap();
}

#[derive(Entity, Debug)]
pub struct TestItem {
    pub id: Option<u32>,
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