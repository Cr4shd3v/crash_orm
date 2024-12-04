use rocket::{async_trait, Build, Rocket};
use rocket::fairing::{Fairing, Info, Kind};
use crash_orm::prelude::*;
use crate::conn::init_connection;

#[derive(Default)]
pub struct CrashOrmDatabaseFairing {
    url: Option<String>,
}

impl CrashOrmDatabaseFairing {
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
        let conn = init_connection(self.url.as_ref().unwrap_or(&std::env::var("DATABASE_URL").unwrap())).await;

        Ok(rocket.manage(conn))
    }
}