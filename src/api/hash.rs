use std::collections::HashMap;

use vaultrs::client::Client;
use vaultrs::error::ClientError;

use crate::api::{
    HashFunction, KeyCurve, KeyResponse, KeysResponse, SignatureResponse, SignaturesResponse,
    requests::{
        CreateKeyRequest, DeleteKeyRequest, ListKeysRequest, ReadKeyRequest, SignBatchRequest,
        SignHashRequest, SignMessageRequest,
    },
};

/// Creates a signer key.
pub async fn create_key(
    client: &impl Client,
    id: &str,
    curve: KeyCurve,
    metadata: HashMap<String, String>,
) -> Result<KeyResponse, ClientError> {
    let request = CreateKeyRequest::builder()
        .id(id)
        .curve(curve)
        .metadata(metadata)
        .build()
        .unwrap();
    vaultrs::api::exec_with_result(client, request).await
}

/// Lists signer keys.
pub async fn list_keys(client: &impl Client) -> Result<KeysResponse, ClientError> {
    vaultrs::api::exec_with_result(client, ListKeysRequest).await
}

/// Reads a signer key.
pub async fn read_key(client: &impl Client, id: &str) -> Result<KeyResponse, ClientError> {
    let request = ReadKeyRequest::builder().id(id).build().unwrap();
    vaultrs::api::exec_with_result(client, request).await
}

/// Deletes a signer key.
pub async fn delete_key(client: &impl Client, id: &str) -> Result<(), ClientError> {
    let request = DeleteKeyRequest::builder().id(id).build().unwrap();
    vaultrs::api::exec_with_empty_result(client, request).await
}

/// Signs a 32-byte hash encoded as hexadecimal.
pub async fn sign_hash(
    client: &impl Client,
    id: &str,
    hash: &str,
) -> Result<SignatureResponse, ClientError> {
    let request = SignHashRequest::builder()
        .id(id)
        .hash(hash)
        .build()
        .unwrap();
    vaultrs::api::exec_with_result(client, request).await
}

/// Signs multiple hexadecimal hashes.
pub async fn sign_batch(
    client: &impl Client,
    id: &str,
    hashes: Vec<String>,
) -> Result<SignaturesResponse, ClientError> {
    let request = SignBatchRequest::builder()
        .id(id)
        .hashes(hashes)
        .build()
        .unwrap();
    vaultrs::api::exec_with_result(client, request).await
}

/// Hashes and signs a hexadecimal message.
pub async fn sign_message(
    client: &impl Client,
    id: &str,
    message: &str,
    hash_function: HashFunction,
) -> Result<SignatureResponse, ClientError> {
    let request = SignMessageRequest::builder()
        .id(id)
        .message(message)
        .hash_function(hash_function)
        .build()
        .unwrap();
    vaultrs::api::exec_with_result(client, request).await
}
