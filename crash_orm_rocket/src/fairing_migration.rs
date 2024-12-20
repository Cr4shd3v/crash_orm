use std::marker::PhantomData;
use rocket::{async_trait, Build, Rocket};
use rocket::fairing::{Fairing, Info, Kind};
use crash_orm::migration::CrashOrmMigrationManager;
use crate::conn::init_connection;

/// Adds the [CrashOrmDatabaseConnection](crash_orm::connection::CrashOrmDatabaseConnection) to the rocket instance.
///
/// You can access the connection with the request guard `&State<CrashOrmDatabaseConnection>`.
///
/// If no url is configured through [CrashOrmDatabaseMigrationFairing::from_url], the environment variable `DATABASE_URL` will be used.
///
/// This version includes migrations. Provide your migration manager as generic parameter.
#[derive(Default)]
pub struct CrashOrmDatabaseMigrationFairing<M: CrashOrmMigrationManager> {
    url: Option<String>,
    phantom: PhantomData<M>,
}

impl<M: CrashOrmMigrationManager> CrashOrmDatabaseMigrationFairing<M> {
    /// Creates the fairing from the desired database URL.
    pub fn from_url(url: impl Into<String>) -> Self {
        Self {
            url: Some(url.into()),
            phantom: PhantomData,
        }
    }
}

#[async_trait]
impl<M: CrashOrmMigrationManager> Fairing for CrashOrmDatabaseMigrationFairing<M> {
    fn info(&self) -> Info {
        Info {
            name: "CrashORM",
            kind: Kind::Ignite,
        }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> rocket::fairing::Result {
        let conn = init_connection(&*self.url.clone().unwrap_or_else(|| {
            std::env::var("DATABASE_URL").unwrap()
        })).await;

        M::migrate_up(&conn).await.expect("Migration failed!");

        Ok(rocket.manage(conn))
    }
}