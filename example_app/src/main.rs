use async_once::AsyncOnce;
use lazy_static::lazy_static;
use crash_orm::crash_orm_derive::{Entity, Schema};
use crash_orm::{DatabaseConnection, Entity, EqualQueryColumn, ManyToOne, Schema};
use crash_orm::tokio_postgres::NoTls;

lazy_static! {
    pub static ref CONN: AsyncOnce<DatabaseConnection> = AsyncOnce::new(async {
        DatabaseConnection::new("postgresql://crash_orm:postgres@localhost/crash_orm_example", NoTls).await.unwrap()
    });
}

#[derive(Entity, Schema, Debug)]
pub struct Author {
    id: Option<u32>,
    name: String,
    age: i32,
}

impl Author {
    pub fn new(name: String, age: i32) -> Self {
        Self {
            id: None,
            name,
            age,
        }
    }
}

#[derive(Entity, Schema, Debug)]
pub struct Book {
    id: Option<u32>,
    author: ManyToOne<Author>,
    title: String,
}

impl Book {
    pub fn new(author: &Author, title: String) -> Self {
        Self {
            id: None,
            author: ManyToOne::new(author.get_id().unwrap()),
            title,
        }
    }
}

#[tokio::main]
async fn main() -> crash_orm::Result<()> {
    let conn = CONN.get().await;
    if !Author::table_exists(conn).await? {
        Author::create_table(conn).await?;
    }

    if !Book::table_exists(conn).await? {
        Book::create_table(conn).await?;
    }

    let mut author = Author::new(String::from("test"), 21);
    author.persist(conn).await?;

    let mut book1 = Book::new(&author, String::from("Lorem ipsum"));
    book1.persist(conn).await?;

    let mut book2 = Book::new(&author, String::from("Rust > C++"));
    book2.persist(conn).await?;

    // Get all books of the author
    let books = Book::query()
        .condition(BookColumn::AUTHOR_ID.equals(author.id.as_ref().unwrap()))
        .execute(conn).await?;

    println!("The following books have been published by {}:", author.name);
    for book in books {
        println!("{}", book.title);
    }

    Book::truncate_table(conn).await?;
    Author::truncate_table(conn).await?;

    Ok(())
}
