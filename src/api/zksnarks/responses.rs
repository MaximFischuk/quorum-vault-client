use serde::{Deserialize, Serialize};

/// Response from executing [CreateZkSnarksAccountRequest][crate::api::zksnarks::requests::CreateZkSnarksAccountRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ZkSnarksAccountResponse {
    pub curve: String,
    pub namespace: String,
    pub public_key: String,
    pub signing_algorithm: String,
}

/// Response from executing [ListZkSnarksAccountsRequest][crate::api::zksnarks::requests::ListZkSnarksAccountsRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ZkSnarksAccountsResponse {
    pub keys: Vec<String>,
}

/// Response from executing [ZkSnarksSignRequest][crate::api::zksnarks::requests::ZkSnarksSignRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ZkSnarksSignResponse {
    pub signature: String,
}
