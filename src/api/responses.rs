use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::api::KeyCurve;

/// Signer key returned by Vault.
#[derive(Debug, Deserialize, Serialize)]
pub struct KeyResponse {
    pub id: String,
    pub curve: KeyCurve,
    #[serde(default)]
    pub metadata: HashMap<String, String>,
    #[serde(default)]
    pub public_key: String,
}

/// Signer key identifiers returned by Vault.
#[derive(Debug, Deserialize, Serialize)]
pub struct KeysResponse {
    pub keys: Vec<String>,
}

/// Signature returned by Vault.
#[derive(Debug, Deserialize, Serialize)]
pub struct SignatureResponse {
    pub signature: String,
}

/// Signatures returned by a batch signing request.
#[derive(Debug, Deserialize, Serialize)]
pub struct SignaturesResponse {
    pub signatures: Vec<String>,
}
