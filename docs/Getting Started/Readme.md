# Getting Started

## Add as dependency
Clone this repo and save it.

Now add crash_orm as dependency in your Cargo.toml:

```toml
crash_orm = "^0.1"
```

OR

```shell
cargo add crash_orm
```

It is recommended to add [tokio](https://crates.io/crates/tokio) as async runtime to actually use the ORM.
However, you can use any async runtime you want.

If you use a different async runtime than tokio, make sure to initialize the tokio_postgres Client on your own!
This library will use tokio in CrashOrmDatabaseConnection.

## Setup Postgres
This ORM only works with Postgres. 
The ORM requires you to have a working installation of Postgres.

To connect to postgres, you will need a connection string based on the following structure:

```
postgresql://<user>:<password>@<netloc>/<dbname>
```

## Create the CrashOrmDatabaseConnection
The first thing you want to do is creating the CrashOrmDatabaseConnection.

```rust
use crash_orm::CrashOrmDatabaseConnection;
use crash_orm::postgres::NoTls;

let conn = CrashOrmDatabaseConnection::new("postgresql://<user>:<password>@<netloc>/<dbname>", NoTls).await
     .expect("Failed to connect to database");
```

This connection is mandatory for all functions on entities.

You should store this variable globally or in case of a web framework like actix you can add it as web::Data.

## Your first Entity
You can now declare your first Entity.

```rust
use crash_orm::derive::{Entity, Schema};

#[derive(Debug, Entity, Schema)]
struct Person {
    id: Option<u32>,
    name: String,
}
```

This will generate a lot of code, if you are curious, you can inspect this struct with 'cargo expand'.

Learn more about entities, their functions and usage [here](../Entity/Readme.md).

