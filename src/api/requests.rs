use std::collections::HashMap;

use derive_builder::Builder;
use rustify_derive::Endpoint;
use serde::{Deserialize, Serialize};

use crate::api::responses::{KeyResponse, KeysResponse, SignatureResponse, SignaturesResponse};

/// Supported signer key curves.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum KeyCurve {
    Secp256k1,
    Ed25519,
    P256,
}

/// Supported message hash functions.
#[derive(Clone, Copy, Debug, Serialize)]
pub enum HashFunction {
    #[serde(rename = "sha256")]
    Sha256,
    #[serde(rename = "keccak256")]
    Keccak256,
    #[serde(rename = "sha512")]
    Sha512,
    #[serde(rename = "sha3-256")]
    Sha3_256,
}

#[derive(Builder, Debug, Endpoint)]
#[endpoint(
    path = "{self.mount}/keys",
    method = "POST",
    response = "KeyResponse",
    builder = "true"
)]
#[builder(setter(into))]
pub struct CreateKeyRequest {
    #[endpoint(skip)]
    pub mount: String,
    pub id: String,
    pub curve: KeyCurve,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Endpoint)]
#[endpoint(path = "{self.mount}/keys", method = "GET", response = "KeysResponse")]
pub struct ListKeysRequest {
    #[endpoint(skip)]
    pub mount: String,
}

#[derive(Builder, Debug, Endpoint)]
#[endpoint(
    path = "{self.mount}/keys/{self.id}",
    method = "GET",
    response = "KeyResponse",
    builder = "true"
)]
#[builder(setter(into))]
pub struct ReadKeyRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub id: String,
}

#[derive(Builder, Debug, Endpoint)]
#[endpoint(
    path = "{self.mount}/keys/{self.id}",
    method = "DELETE",
    builder = "true"
)]
#[builder(setter(into))]
pub struct DeleteKeyRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub id: String,
}

#[derive(Builder, Debug, Endpoint)]
#[endpoint(
    path = "{self.mount}/keys/{self.id}/sign/hash",
    method = "POST",
    response = "SignatureResponse",
    builder = "true"
)]
#[builder(setter(into))]
pub struct SignHashRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub id: String,
    pub hash: String,
}

#[derive(Builder, Debug, Endpoint)]
#[endpoint(
    path = "{self.mount}/keys/{self.id}/sign/batch",
    method = "POST",
    response = "SignaturesResponse",
    builder = "true"
)]
#[builder(setter(into))]
pub struct SignBatchRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub id: String,
    pub hashes: Vec<String>,
}

#[derive(Builder, Debug, Endpoint)]
#[endpoint(
    path = "{self.mount}/keys/{self.id}/sign/message",
    method = "POST",
    response = "SignatureResponse",
    builder = "true"
)]
#[builder(setter(into))]
pub struct SignMessageRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub id: String,
    pub message: String,
    pub hash_function: HashFunction,
}
