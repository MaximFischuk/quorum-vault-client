use crate::api::ethereum::responses::{EthereumAccountResponse, EthereumAccountsResponse, EthereumSignTransactionResponse};
use crate::error::ClientError;
use vaultrs::client::Client;
use web3::types::TransactionRequest;
use crate::api::ethereum::requests::{ListEthereumAccountsRequest, ReadEthereumAccountRequest, SignEthereumTransactionRequest};

pub mod ethereum;

pub async fn list_accouns(
    client: &impl Client,
    mount: &str,
) -> Result<EthereumAccountsResponse, ClientError> {
    let request = ListEthereumAccountsRequest::builder()
        .mount(mount)
        .build()
        .unwrap();
    vaultrs::api::exec_with_result(client, request)
        .await
        .map_err(Into::into)
}

pub async fn read_account(
    client: &impl Client,
    mount: &str,
    address: &str,
) -> Result<EthereumAccountResponse, ClientError> {
    let request = ReadEthereumAccountRequest::builder()
        .mount(mount)
        .address(address)
        .build()
        .unwrap();
    vaultrs::api::exec_with_result(client, request)
        .await
        .map_err(Into::into)
}

pub async fn sign_transaction(
    client: &impl Client,
    mount: &str,
    transaction: TransactionRequest,
) -> Result<EthereumSignTransactionResponse, ClientError> {
    let address = format!("{:?}", transaction.from);
    let checksummed = eth_checksum::checksum(&address);
    let request = SignEthereumTransactionRequest::builder()
        .mount(mount)
        .address(checksummed)
        .chain_id("1".to_string())
        .amount(transaction.value.unwrap_or_default().to_string())
        .gas_limit(transaction.gas.map(|v| v.as_u64()).unwrap_or(21000))
        .gas_price(transaction.gas_price.unwrap_or_default().to_string())
        .nonce(transaction.nonce.unwrap_or_default().as_u64())
        .to(format!("{:?}", transaction.to.unwrap_or_default()))
        .data(transaction.data.unwrap_or_default())
        .build()
        .unwrap();
    vaultrs::api::exec_with_result(client, request)
        .await
        .map_err(Into::into)
}
