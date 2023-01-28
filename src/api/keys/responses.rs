use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Response from executing [CreateKeyRequest][crate::api::keys::requests::CreateKeyRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct KeyResponse {
    pub created_at: String,
    pub curve: String,
    pub id: String,
    pub namespace: String,
    pub public_key: String,
    pub signing_algorithm: String,
    pub tags: HashMap<String, String>,
    pub updated_at: String,
    pub version: u64,
}

/// Response from executing [ListKeysRequest][crate::api::keys::requests::ListKeysRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct KeysResponse {
    pub keys: Vec<String>,
}

/// Response from executing [SignRequest][crate::api::keys::requests::SignRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct SignResponse {
    pub signature: String,
}
