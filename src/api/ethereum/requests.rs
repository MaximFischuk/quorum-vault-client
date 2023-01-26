use crate::api::ethereum::responses::{
    EthereumAccountResponse, EthereumAccountsResponse, EthereumSignTransactionResponse,
};
use rustify_derive::Endpoint;
use web3::types::Bytes;

/// ## Create Ethereum Account
/// This endpoint creates a new Ethereum account.
///
/// * Path: {self.mount}/ethereum/accounts
/// * Method: POST
/// * Response: [EthereumAccountResponse]
#[derive(Builder, Debug, Endpoint)]
#[endpoint(
    path = "{self.mount}/ethereum/accounts",
    method = "POST",
    response = "EthereumAccountResponse",
    builder = "true"
)]
#[builder(setter(into))]
pub struct CreateEthereumAccountRequest {
    #[endpoint(skip)]
    pub mount: String,
}

/// ## Read Ethereum Account
/// This endpoint an Ethereum account by Address.
///
/// * Path: {self.mount}/ethereum/accounts/{self.address}
/// * Method: GET
/// * Response: [EthereumAccountResponse]
#[derive(Builder, Debug, Endpoint)]
#[endpoint(
    path = "{self.mount}/ethereum/accounts/{self.address}",
    method = "GET",
    response = "EthereumAccountResponse",
    builder = "true"
)]
#[builder(setter(into))]
pub struct ReadEthereumAccountRequest {
    #[endpoint(skip)]
    pub mount: String,

    #[endpoint(skip)]
    pub address: String,
}

/// ## List Ethereum Accounts
/// This endpoint gets all Ethereum accounts.
///
/// * Path: {self.mount}/ethereum/accounts
/// * Method: GET
/// * Response: [EthereumAccountsResponse]
#[derive(Builder, Debug, Endpoint)]
#[endpoint(
    path = "{self.mount}/ethereum/accounts",
    method = "GET",
    response = "EthereumAccountsResponse",
    builder = "true"
)]
#[builder(setter(into))]
pub struct ListEthereumAccountsRequest {
    #[endpoint(skip)]
    pub mount: String,
}

/// ## Sign Ethereum Transaction
/// This endpoint signs an Ethereum transaction.
///
/// * Path: {self.mount}/ethereum/sign/{self.address}
/// * Method: POST
/// * Response: [EthereumSignTransactionResponse]
#[derive(Builder, Debug, Endpoint)]
#[endpoint(
    path = "{self.mount}/ethereum/accounts/{self.address}/sign-transaction",
    method = "POST",
    response = "EthereumSignTransactionResponse",
    builder = "true"
)]
#[builder(setter(into))]
pub struct SignEthereumTransactionRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub address: String,
    #[endpoint(body)]
    pub chain_id: String,
    #[endpoint(body)]
    pub amount: String,
    #[endpoint(body)]
    pub data: Bytes,
    #[endpoint(body)]
    pub gas_limit: u64,
    #[endpoint(body)]
    pub gas_price: String,
    #[endpoint(body)]
    pub nonce: u64,
    #[endpoint(body)]
    pub to: String,
}
