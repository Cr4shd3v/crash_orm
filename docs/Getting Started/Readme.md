# Getting Started

## Add as dependency
Clone this repo and save it.

Now add crash_orm as dependency in your Cargo.toml:

```toml
crash_orm = { path = "relative/path/to/crash_orm/folder" }
```

It is recommended to add [tokio](https://crates.io/crates/tokio) as async runtime to actually use the ORM.
However, you can use any async runtime you want.

## Setup Postgres
This ORM only works with Postgres. 
The ORM requires you to have a working installation of Postgres.

To connect to postgres, you will need a connection string based on the following structure:

```
postgresql://<user>:<password>@<netloc>/<dbname>
```

## Create the DatabaseConnection
The first thing you want to do is creating the DatabaseConnection.

```rust
use crash_orm::DatabaseConnection;

let conn = DatabaseConnection::new("postgresql://<user>:<password>@<netloc>/<dbname>").await?;
```

This connection is mandatory for all functions on entities.

However, the variable is only available in the current context.
If you need a global variable for the database connection, take a look at the [Example App](../../example_app/src/main.rs).

## Your first Entity
You can now start by declaring your first Entity.

```rust

```


