# Migration

## Setup
First, create a new lib crate in your project folder.
```shell
cargo new migration --lib
```

Next, add crash_orm_migration and crash_orm as dependency for this new crate and reexport crash_orm_migration::CrashOrmMigrationManager.

Inside the lib.rs, create your migration manager:

```rust
extern crate crash_orm_migration;
pub use crash_orm_migration::CrashOrmMigrationManager;

use crash_orm::DatabaseConnection;
use crash_orm_migration::Migration;

pub struct MigrationManager;

impl<T: DatabaseConnection> CrashOrmMigrationManager<T> for MigrationManager {
    fn get_migrations() -> Vec<Box<dyn Migration<T>>> {
        vec![
            
        ]
    }
}
```

You can now add your migrations inside the get_migrations function like this:

```rust
vec![
    Box::new(ExampleMigraton),
]
```

## Create Migration

As of now you have to manually create migrations. However, it is planned to generate them in the future.

```rust
use crash_orm::async_trait::async_trait;
use crash_orm::{DatabaseConnection, Schema};
use crash_orm_migration::Migration;

pub struct ExampleMigration;

#[async_trait]
impl<T: DatabaseConnection> Migration<T> for ExampleMigration {
    async fn up(&self, conn: &T) -> crash_orm::Result<()> {
        // UP, like User::create_table_if_not_exists(conn).await?;
    }

    async fn down(&self, conn: &T) -> crash_orm::Result<()> {
        // DOWN, like User::drop_table(conn).await?;
    }

    fn get_name(&self) -> String {
        String::from("ExampleMigration") // The name MUST ALWAYS be UNIQUE
    }
}
```

IMPORTANT: The returned string from get_name MUST be UNIQUE across ALL migrations.

## Execute Migrations
On Startup of your app, you should call the migrate_up method of your migration manager:

```rust
MigrationManager::migrate_up(conn).await?;
```

You should take care of the potential Err returned by this function since this likely means that parts of your migration failed.

migrate_up terminates after the first error, no following statements are executed.