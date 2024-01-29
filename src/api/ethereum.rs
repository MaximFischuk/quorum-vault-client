use crate::api::ethereum::requests::{
    CreateEthereumAccountRequest, ImportPrivateKeyRequest, ListEthereumAccountsRequest,
    ReadEthereumAccountRequest, SignEthereumTransactionRequest,
};
use crate::api::ethereum::responses::{
    EthereumAccountResponse, EthereumAccountsResponse, EthereumSignTransactionResponse,
};

use vaultrs::client::Client;
use vaultrs::error::ClientError;
use web3::types::{Address, TransactionRequest};

use self::requests::EthereumSignRequest;
use self::responses::EthereumSignResponse;

pub mod requests;
pub mod responses;

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
pub async fn list_accounts(
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
    let address = format!("{address:?}");
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

/// Import a Private Key
/// See [ImportPrivateKeyRequest]
pub async fn import_private_key(
    client: &impl Client,
    mount: &str,
    private_key: &str,
) -> Result<EthereumAccountResponse, ClientError> {
    let request = ImportPrivateKeyRequest::builder()
        .mount(mount)
        .private_key(private_key)
        .build()
        .unwrap();
    vaultrs::api::exec_with_result(client, request)
        .await
        .map_err(Into::into)
}

/// Sign a message with an Ethereum account.
/// See [SignEthereumRequest]
pub async fn sign(
    client: &impl Client,
    mount: &str,
    address: Address,
    data: &[u8],
) -> Result<EthereumSignResponse, ClientError> {
    let address = format!("{address:?}");
    let checksummed = eth_checksum::checksum(&address);
    let request = EthereumSignRequest::builder()
        .mount(mount)
        .address(checksummed)
        .data(format!("0x{}", hex::encode(data)))
        .build()
        .unwrap();
    vaultrs::api::exec_with_result(client, request)
        .await
        .map_err(Into::into)
}
