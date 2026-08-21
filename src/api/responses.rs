use std::collections::HashMap;

use serde::{Deserialize, Deserializer, Serialize};

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
    #[serde(default, deserialize_with = "deserialize_keys")]
    pub keys: Vec<String>,
}

fn deserialize_keys<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    Option::<Vec<String>>::deserialize(deserializer).map(Option::unwrap_or_default)
}

/// Signature returned by Vault.
#[derive(Debug, Deserialize, Serialize)]
pub struct SignatureResponse {
    pub signature: String,
}

/// Signed Ethereum transaction returned by Vault.
#[derive(Debug, Deserialize, Serialize)]
pub struct SignedEthereumTransactionResponse {
    pub signed_transaction: String,
    pub transaction_hash: String,
}

/// Signatures returned by a batch signing request.
#[derive(Debug, Deserialize, Serialize)]
pub struct SignaturesResponse {
    pub signatures: Vec<String>,
}
