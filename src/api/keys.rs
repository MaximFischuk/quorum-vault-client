use std::collections::HashMap;

use base64::Engine;
use vaultrs::client::Client;
use vaultrs::error::ClientError;

use crate::api::keys::requests::{
    CreateKeyRequest, DestroyKeyRequest, ImportKeyRequest, ListKeysRequest, ReadKeyRequest,
    SignRequest, UpdateKeyTagsRequest,
};
use crate::api::keys::responses::{KeyResponse, KeysResponse, SignResponse};

pub mod requests;
pub mod responses;

/// Key crypto algorithm.
pub enum KeyCryptoAlgorithm {
    Secp256k1,
    Babyjubjub,
}

impl KeyCryptoAlgorithm {
    /// Returns the curve name for the algorithm
    pub fn curve(&self) -> &'static str {
        match self {
            KeyCryptoAlgorithm::Secp256k1 => "secp256k1",
            KeyCryptoAlgorithm::Babyjubjub => "babyjubjub",
        }
    }

    /// Returns the signing algorithm name for the algorithm
    pub fn signing_algorithm(&self) -> &'static str {
        match self {
            KeyCryptoAlgorithm::Secp256k1 => "ecdsa",
            KeyCryptoAlgorithm::Babyjubjub => "eddsa",
        }
    }
}

/// Create a new Key
/// See [CreateKeyRequest]
pub async fn create_key(
    client: &impl Client,
    mount: &str,
    id: &str,
    algorithm: KeyCryptoAlgorithm,
    tags: HashMap<String, String>,
) -> Result<KeyResponse, ClientError> {
    let request = CreateKeyRequest::builder()
        .mount(mount)
        .id(id)
        .signing_algorithm(algorithm.signing_algorithm().to_string())
        .curve(algorithm.curve().to_string())
        .tags(tags)
        .build()
        .unwrap();
    vaultrs::api::exec_with_result(client, request)
        .await
        .map_err(Into::into)
}

/// Read a Key
/// See [ReadKeyRequest]
pub async fn read_key(
    client: &impl Client,
    mount: &str,
    id: &str,
) -> Result<KeyResponse, ClientError> {
    let request = ReadKeyRequest::builder()
        .mount(mount)
        .id(id)
        .build()
        .unwrap();
    vaultrs::api::exec_with_result(client, request)
        .await
        .map_err(Into::into)
}

/// List Keys
/// See [ListKeysRequest]
pub async fn list_keys(client: &impl Client, mount: &str) -> Result<KeysResponse, ClientError> {
    let request = ListKeysRequest::builder().mount(mount).build().unwrap();
    vaultrs::api::exec_with_result(client, request)
        .await
        .map_err(Into::into)
}

/// Update a Key tags
/// See [UpdateKeyTagsRequest]
pub async fn update_key_tags(
    client: &impl Client,
    mount: &str,
    id: &str,
    tags: HashMap<String, String>,
) -> Result<KeyResponse, ClientError> {
    let request = UpdateKeyTagsRequest::builder()
        .mount(mount)
        .id(id)
        .tags(tags)
        .build()
        .unwrap();
    vaultrs::api::exec_with_result(client, request)
        .await
        .map_err(Into::into)
}

/// Destroy a Key
/// See [DestroyKeyRequest]
pub async fn destroy_key(client: &impl Client, mount: &str, id: &str) -> Result<(), ClientError> {
    let request = DestroyKeyRequest::builder()
        .mount(mount)
        .id(id)
        .build()
        .unwrap();
    vaultrs::api::exec_with_empty_result(client, request)
        .await
        .map_err(Into::into)
}

/// Import a Key
/// See [ImportKeyRequest]
pub async fn import_key(
    client: &impl Client,
    mount: &str,
    id: &str,
    algorithm: KeyCryptoAlgorithm,
    tags: HashMap<String, String>,
    private_key: &str,
) -> Result<KeyResponse, ClientError> {
    let request = ImportKeyRequest::builder()
        .mount(mount)
        .id(id)
        .signing_algorithm(algorithm.signing_algorithm().to_string())
        .curve(algorithm.curve().to_string())
        .tags(tags)
        .private_key(private_key)
        .build()
        .unwrap();
    vaultrs::api::exec_with_result(client, request)
        .await
        .map_err(Into::into)
}

/// Sign a message
/// See [SignRequest]
pub async fn sign(
    client: &impl Client,
    mount: &str,
    id: &str,
    data: &[u8],
) -> Result<SignResponse, ClientError> {
    let hash = web3::signing::keccak256(data);
    let encoded = base64::prelude::BASE64_URL_SAFE.encode(hash);
    let request = SignRequest::builder()
        .mount(mount)
        .id(id)
        .data(encoded)
        .build()
        .unwrap();
    vaultrs::api::exec_with_result(client, request)
        .await
        .map_err(Into::into)
}

/// Sign a message
/// Data must be a 32 byte hash
/// See [SignRequest]
pub async fn sign_hash(
    client: &impl Client,
    mount: &str,
    id: &str,
    data: [u8; 32],
) -> Result<SignResponse, ClientError> {
    let encoded = base64::prelude::BASE64_URL_SAFE.encode(data);
    let request = SignRequest::builder()
        .mount(mount)
        .id(id)
        .data(encoded)
        .build()
        .unwrap();
    vaultrs::api::exec_with_result(client, request)
        .await
        .map_err(Into::into)
}
