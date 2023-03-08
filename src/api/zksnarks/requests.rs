use crate::api::zksnarks::responses::*;
use rustify_derive::Endpoint;

/// ## Create Zk-Snarks Account
/// This endpoint creates a new Zk-Snarks account.
///
/// * Path: /zk-snarks/accounts
/// * Method: POST
/// * Response: [ZkSnarksAccountResponse]
#[derive(Builder, Debug, Endpoint)]
#[endpoint(
    path = "{self.mount}/zk-snarks/accounts",
    method = "POST",
    response = "ZkSnarksAccountResponse",
    builder = "true"
)]
#[builder(setter(into))]
pub struct CreateZkSnarksAccountRequest {
    #[endpoint(skip)]
    pub mount: String,
}

/// ## Read Zk-Snarks Account
/// This endpoint gets a Zk-Snarks account by ID.
///
/// * Path: /zk-snarks/accounts/{self.id}
/// * Method: GET
/// * Response: [ZkSnarksAccountResponse]
#[derive(Builder, Debug, Endpoint)]
#[endpoint(
    path = "{self.mount}/zk-snarks/accounts/{self.id}",
    method = "GET",
    response = "ZkSnarksAccountResponse",
    builder = "true"
)]
#[builder(setter(into))]
pub struct ReadZkSnarksAccountRequest {
    #[endpoint(skip)]
    pub mount: String,
    pub id: String,
}

/// ## List Zk-Snarks Accounts
/// This endpoint gets all Zk-Snarks accounts.
///
/// * Path: /zk-snarks/accounts
/// * Method: GET
/// * Response: [ZkSnarksAccountsResponse]
#[derive(Builder, Debug, Endpoint)]
#[endpoint(
    path = "{self.mount}/zk-snarks/accounts",
    method = "GET",
    response = "ZkSnarksAccountsResponse",
    builder = "true"
)]
#[builder(setter(into))]
pub struct ListZkSnarksAccountsRequest {
    #[endpoint(skip)]
    pub mount: String,
}

/// ## Sign data with Zk-Snarks Account
/// This endpoint signs data with a Zk-Snarks account.
///
/// * Path: /zk-snarks/accounts/{self.id}/sign
/// * Method: POST
/// * Response: [ZkSnarksSignResponse]
#[derive(Builder, Debug, Endpoint)]
#[endpoint(
    path = "{self.mount}/zk-snarks/accounts/{self.id}/sign",
    method = "POST",
    response = "ZkSnarksSignResponse",
    builder = "true"
)]
#[builder(setter(into))]
pub struct ZkSnarksSignRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub id: String,
    #[endpoint(body)]
    pub data: String,
}
