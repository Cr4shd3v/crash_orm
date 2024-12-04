use rocket::{async_trait, Build, Rocket};
use rocket::fairing::{Fairing, Info, Kind};
use crate::conn::init_connection;

/// Adds the [CrashOrmDatabaseConnection](crash_orm::connection::CrashOrmDatabaseConnection) to the rocket instance.
///
/// You can access the connection with the request guard `&State<CrashOrmDatabaseConnection>`.
///
/// If no url is configured through [CrashOrmDatabaseFairing::from_url], the environment variable `DATABASE_URL` will be used.
#[derive(Default)]
pub struct CrashOrmDatabaseFairing {
    url: Option<String>,
}

impl CrashOrmDatabaseFairing {
    /// Creates the fairing from the desired database URL.
    pub fn from_url(url: impl Into<String>) -> Self {
        Self {
            url: Some(url.into()),
        }
    }
}

#[async_trait]
impl Fairing for CrashOrmDatabaseFairing {
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

        Ok(rocket.manage(conn))
    }
}