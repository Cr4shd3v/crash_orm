use std::fs;
use std::process::Command;
use cargo_toml::{Dependency, DependencyDetail, InheritedDependencyDetail};
use clap::Parser;
use crate::config::CrashOrmToml;

#[derive(Parser, Debug)]
pub struct Init {

}

impl Init {
    pub fn run(self) {
        let config = CrashOrmToml::load_or_create();

        if fs::metadata(&config.migration_project_path).is_err() {
            println!("Run `cargo new`...");
            Command::new("cargo")
                .args(&["new", &*config.migration_project_path, "--lib"])
                .output().expect("Failed to run cargo new");
        }

        let lib_rs_path = format!("{}/src/lib.rs", config.migration_project_path);
        let migration_cargo_toml_path = format!("{}/Cargo.toml", config.migration_project_path);
        let workspace_toml_path = "Cargo.toml";

        let mut migration_manifest = cargo_toml::Manifest::from_path(&migration_cargo_toml_path).unwrap();
        let mut workspace_manifest = cargo_toml::Manifest::from_path(workspace_toml_path).unwrap();

        if let Some(workspace) = workspace_manifest.workspace.as_mut() {
            if workspace.dependencies.get("crash_orm").is_none() {
                workspace.dependencies.insert("crash_orm".to_string(), Dependency::Simple(env!("CARGO_PKG_VERSION").to_string()));
            }

            workspace.members.push(config.migration_project_path.split("/").last().unwrap().to_string());

            fs::write(workspace_toml_path, toml::to_string_pretty(&migration_manifest).unwrap()).unwrap();

            migration_manifest.dependencies.insert("crash_orm".to_string(), Dependency::Inherited(InheritedDependencyDetail {
                workspace: true,
                features: vec!["migration".to_string()],
                ..Default::default()
            }));
        } else {
            migration_manifest.dependencies.insert("crash_orm".to_string(), Dependency::Detailed(Box::new(DependencyDetail {
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
                features: vec!["migration".to_string()],
                ..Default::default()
            })));
        }

        fs::write(workspace_toml_path, toml::to_string_pretty(&migration_manifest).unwrap()).unwrap();

        let manager_template = include_str!("../../templates/manager.rs.template");
        fs::write(&lib_rs_path, manager_template).unwrap();
    }
}