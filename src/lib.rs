//! # Quorum Vault Client
//!
//! Rust client for Vault signer plugin.
//!
//! ```no_run
//! use std::collections::HashMap;
//!
//! use quorum_vault_client::{
//!     api::signer::{self, KeyCurve},
//!     VaultClient, VaultClientSettingsBuilder,
//! };
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = VaultClient::new(
//!     VaultClientSettingsBuilder::default()
//!         .address("http://127.0.0.1:8200")
//!         .token("TOKEN")
//!         .build()?,
//! )?;
//!
//! let key = signer::create_key(
//!     &client,
//!     "secp256k1",
//!     KeyCurve::Secp256k1,
//!     HashMap::from([(String::from("owner"), String::from("Alice"))]),
//! )
//! .await?;
//! # Ok(())
//! # }
//! ```
#![forbid(unsafe_code)]
#![forbid(clippy::unwrap_used, clippy::expect_used)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]

pub mod api;

pub use vaultrs::client::{Client, VaultClient, VaultClientSettingsBuilder};
