use crate::api::zksnarks::requests::{
    CreateZkSnarksAccountRequest, ListZkSnarksAccountsRequest, ReadZkSnarksAccountRequest,
    ZkSnarksSignRequest,
};
use crate::api::zksnarks::responses::{
    ZkSnarksAccountResponse, ZkSnarksAccountsResponse, ZkSnarksSignResponse,
};
use crate::error::ClientError;
use crate::H256;
use vaultrs::client::Client;

pub mod requests;
pub mod responses;

/// Create a zk-SNARKs account (eddsa)
/// See [CreateZkSnarksAccountRequest]
pub async fn create_zksnarks_account(
    client: &impl Client,
    mount: &str,
) -> Result<ZkSnarksAccountResponse, ClientError> {
    let request = CreateZkSnarksAccountRequest::builder()
        .mount(mount)
        .build()
        .unwrap();
    vaultrs::api::exec_with_result(client, request)
        .await
        .map_err(Into::into)
}

/// Read a zk-SNARKs account
/// See [ReadZkSnarksAccountRequest]
pub async fn read_zksnarks_account(
    client: &impl Client,
    mount: &str,
    id: &str,
) -> Result<ZkSnarksAccountResponse, ClientError> {
    let request = ReadZkSnarksAccountRequest::builder()
        .mount(mount)
        .id(id)
        .build()
        .unwrap();
    vaultrs::api::exec_with_result(client, request)
        .await
        .map_err(Into::into)
}

/// List zk-SNARKs accounts
/// See [ListZkSnarksAccountsRequest]
pub async fn list_zksnarks_accounts(
    client: &impl Client,
    mount: &str,
) -> Result<ZkSnarksAccountsResponse, ClientError> {
    let request = ListZkSnarksAccountsRequest::builder()
        .mount(mount)
        .build()
        .unwrap();
    vaultrs::api::exec_with_result(client, request)
        .await
        .map_err(Into::into)
}

/// Sign a message with a zk-SNARKs account (eddsa)
/// See [ZkSnarksSignResponse]
pub async fn zksnarks_sign(
    client: &impl Client,
    mount: &str,
    id: &str,
    data: &[u8],
) -> Result<ZkSnarksSignResponse, ClientError> {
    let hash = web3::signing::keccak256(data);
    let hex = H256::from(hash);
    let encoded = format!("{:?}", hex);
    let request = ZkSnarksSignRequest::builder()
        .mount(mount)
        .id(id)
        .data(encoded)
        .build()
        .unwrap();
    vaultrs::api::exec_with_result(client, request)
        .await
        .map_err(Into::into)
}

/// Sign a message with a zk-SNARKs account (eddsa)
/// Data must be a 32 byte hash
/// See [ZkSnarksSignResponse]
pub async fn zksnarks_sign_hash(
    client: &impl Client,
    mount: &str,
    id: &str,
    data: [u8; 32],
) -> Result<ZkSnarksSignResponse, ClientError> {
    let hex = H256::from(data);
    let encoded = format!("{:?}", hex);
    let request = ZkSnarksSignRequest::builder()
        .mount(mount)
        .id(id)
        .data(encoded)
        .build()
        .unwrap();
    vaultrs::api::exec_with_result(client, request)
        .await
        .map_err(Into::into)
}
