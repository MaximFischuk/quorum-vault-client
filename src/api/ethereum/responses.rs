use serde::{Deserialize, Serialize};
use web3::types::Address;

/// Response from executing
/// [ReadEthereumAccountRequest][crate::api::ethereum::requests::ReadEthereumAccountRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct EthereumAccountResponse {
    pub address: Address,
    pub compressed_public_key: String,
    pub public_key: String,
    pub namespace: String,
}

/// Response from executing
/// [SignEthereumTransactionRequest][crate::api::ethereum::requests::SignEthereumTransactionRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct EthereumSignTransactionResponse {
    pub signature: String,
}

/// Response from executing
/// [ListEthereumAccountsRequest][crate::api::ethereum::requests::ListEthereumAccountsRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct EthereumAccountsResponse {
    pub keys: Vec<Address>,
}
