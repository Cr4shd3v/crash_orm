use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CrashOrmToml {
    pub migration_project_path: String,
}

impl Default for CrashOrmToml {
    fn default() -> Self {
        Self {
            migration_project_path: "migrations".to_string(),
        }
    }
}

impl CrashOrmToml {
    pub const FILE_NAME: &'static str = "crash_orm.toml";

    pub fn try_load() -> Option<Self> {
        if fs::metadata(Self::FILE_NAME).is_ok() {
            let content = fs::read_to_string(Self::FILE_NAME).unwrap();
            Some(toml::from_str(&*content).unwrap())
        } else {
            None
        }
    }

    pub fn load_or_create() -> Self {
        if let Some(config) = Self::try_load() {
            config
        } else {
            let config = CrashOrmToml::default();
            fs::write(Self::FILE_NAME, toml::to_string(&config).unwrap()).unwrap();
            config
        }
    }
}
