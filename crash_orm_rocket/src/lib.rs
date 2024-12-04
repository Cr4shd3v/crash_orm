#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]

#[cfg(not(feature = "migration"))]
mod fairing;
mod fairing_migration;
mod conn;

#[cfg(not(feature = "migration"))]
pub use fairing::*;

#[cfg(feature = "migration")]
pub use fairing_migration::*;