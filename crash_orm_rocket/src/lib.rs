//! This crate integrates crash_orm into rocket.
//!
//! To use this crate, attach the corresponding [CrashOrmDatabaseFairing] to your rocket instance.
//!
//! You can then access your configured connection with the request guard `&State<CrashOrmDatabaseConnection>`.

#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]

#[cfg(not(feature = "migration"))]
mod fairing;
#[cfg(feature = "migration")]
mod fairing_migration;
mod conn;

#[cfg(not(feature = "migration"))]
pub use fairing::*;

#[cfg(feature = "migration")]
pub use fairing_migration::*;