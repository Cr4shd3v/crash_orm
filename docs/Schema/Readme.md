# Schema
The Schema trait is useful to manage the table itself with operations like create, delete or truncate.

This trait can be derived via macro.

Example:
```rust
use crash_orm::crash_orm_derive::{Entity, Schema};

#[derive(Entity, Debug, Schema)]
struct TestEntity {
    id: Option<u32>,
}
```

## Check table exists
A simple function returning a bool whether a table exists or not.

```rust
let exists = TestEntity::table_exists(&conn).await?;
```

## Create Table
Creates the table based on the properties of the entity.

```rust
TestEntity::create_table(&conn).await?;
```

## Drop Table
Drop the table of the entity. This deletes the table itself from the database.

```rust
TestEntity::drop_table(&conn).await?;
```

## Truncate Table
Truncate the table of the entity. This deletes the content of the table from the database.

```rust
TestEntity::truncate_table(&conn).await?;
```


