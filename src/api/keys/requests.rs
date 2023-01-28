use crate::api::keys::responses::{KeyResponse, KeysResponse, SignResponse};
use rustify_derive::Endpoint;
use std::collections::HashMap;

/// ## Create Key
/// This endpoint creates a new key with given crypto algorithm and curve.
///
/// * Path: {self.mount}/keys
/// * Method: POST
/// * Response: [KeyResponse]
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
    pub signing_algorithm: String,
    pub curve: String,
    pub tags: HashMap<String, String>,
    pub id: String,
}

/// ## Read Key
/// This endpoint gets a key by ID.
///
/// * Path: {self.mount}/keys/{self.id}
/// * Method: GET
/// * Response: [KeyResponse]
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

/// ## List Keys
/// This endpoint gets all keys.
///
/// * Path: {self.mount}/keys
/// * Method: GET
/// * Response: [KeysResponse]
#[derive(Builder, Debug, Endpoint)]
#[endpoint(
    path = "{self.mount}/keys",
    method = "GET",
    response = "KeysResponse",
    builder = "true"
)]
#[builder(setter(into))]
pub struct ListKeysRequest {
    #[endpoint(skip)]
    pub mount: String,
}

/// ## Destroy Key
/// This endpoint destroys a key by ID.
///
/// * Path: {self.mount}/keys/{self.id}/destroy
/// * Method: DELETE
#[derive(Builder, Debug, Endpoint)]
#[endpoint(
    path = "{self.mount}/keys/{self.id}/destroy",
    method = "DELETE",
    builder = "true"
)]
#[builder(setter(into))]
pub struct DestroyKeyRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub id: String,
}

/// ## Import Key
/// This endpoint imports a key with given crypto algorithm and curve.
///
/// * Path: {self.mount}/keys/import
/// * Method: POST
/// * Response: [KeyResponse]
#[derive(Builder, Debug, Endpoint)]
#[endpoint(
    path = "{self.mount}/keys/import",
    method = "POST",
    response = "KeyResponse",
    builder = "true"
)]
#[builder(setter(into))]
pub struct ImportKeyRequest {
    #[endpoint(skip)]
    pub mount: String,
    pub signing_algorithm: String,
    pub curve: String,
    pub tags: HashMap<String, String>,
    pub private_key: String,
    pub id: String,
}

/// ## Update tags of Key
/// This endpoint updates the tags of a key by ID.
///
/// * Path: {self.mount}/keys/{self.id}
/// * Method: POST
/// * Response: [KeyResponse]
#[derive(Builder, Debug, Endpoint)]
#[endpoint(
    path = "{self.mount}/keys/{self.id}",
    method = "POST",
    response = "KeyResponse",
    builder = "true"
)]
#[builder(setter(into))]
pub struct UpdateKeyTagsRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub id: String,
    pub tags: HashMap<String, String>,
}

/// ## Sign Data
/// This endpoint signs data with a key by ID.
/// The data is base64 encoded before signing.
///
/// * Path: {self.mount}/keys/{self.id}/sign
/// * Method: POST
/// * Response: [SignResponse]
#[derive(Builder, Debug, Endpoint)]
#[endpoint(
    path = "{self.mount}/keys/{self.id}/sign",
    method = "POST",
    response = "SignResponse",
    builder = "true"
)]
#[builder(setter(into))]
pub struct SignRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub id: String,
    pub data: String,
}
