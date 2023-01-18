use crate::api::ethereum::responses::{EthereumAccountResponse, EthereumAccountsResponse, EthereumSignTransactionResponse};
use crate::error::ClientError;
use vaultrs::client::Client;
use web3::types::{Address, TransactionRequest};
use crate::api::ethereum::requests::{CreateEthereumAccountRequest, ListEthereumAccountsRequest, ReadEthereumAccountRequest, SignEthereumTransactionRequest};

pub mod ethereum;

/// Create a new Ethereum account.
///
/// See [CreateEthereumAccountRequest]
pub async fn create_account(
    client: &impl Client,
    mount: &str,
) -> Result<EthereumAccountResponse, ClientError> {
    let request = CreateEthereumAccountRequest::builder()
        .mount(mount)
        .build()
        .unwrap();
    vaultrs::api::exec_with_result(client, request)
        .await
        .map_err(Into::into)
}

/// List Ethereum accounts.
///
/// See [ListEthereumAccountsRequest]
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

/// Read an Ethereum account.
///
/// See [ReadEthereumAccountRequest]
pub async fn read_account(
    client: &impl Client,
    mount: &str,
    address: Address,
) -> Result<EthereumAccountResponse, ClientError> {
    let address = format!("{:?}", address);
    let checksummed = eth_checksum::checksum(&address);
    let request = ReadEthereumAccountRequest::builder()
        .mount(mount)
        .address(checksummed)
        .build()
        .unwrap();
    vaultrs::api::exec_with_result(client, request)
        .await
        .map_err(Into::into)
}

/// Sign an Ethereum transaction.
///
/// See [SignEthereumTransactionRequest]
pub async fn sign_transaction(
    client: &impl Client,
    mount: &str,
    chain_id: u64,
    transaction: TransactionRequest,
) -> Result<EthereumSignTransactionResponse, ClientError> {
    let address = format!("{:?}", transaction.from);
    let checksummed = eth_checksum::checksum(&address);
    let request = SignEthereumTransactionRequest::builder()
        .mount(mount)
        .address(checksummed)
        .chain_id(chain_id.to_string())
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
