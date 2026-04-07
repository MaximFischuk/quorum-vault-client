use crate::api::ethereum::requests::{
    CreateEthereumAccountRequest, ImportPrivateKeyRequest, ListEthereumAccountsRequest,
    ReadEthereumAccountRequest, SignEthereumTransactionRequest,
};
use crate::api::ethereum::responses::{
    EthereumAccountResponse, EthereumAccountsResponse, EthereumSignTransactionResponse,
};

use alloy_primitives::{Address, B256, TxKind};
use alloy_rpc_types_eth::TransactionRequest;
use vaultrs::client::Client;
use vaultrs::error::ClientError;

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
    vaultrs::api::exec_with_result(client, request).await
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
    vaultrs::api::exec_with_result(client, request).await
}

/// Read an Ethereum account.
///
/// See [ReadEthereumAccountRequest]
pub async fn read_account(
    client: &impl Client,
    mount: &str,
    address: Address,
) -> Result<EthereumAccountResponse, ClientError> {
    let checksummed = address.to_checksum(None);
    let request = ReadEthereumAccountRequest::builder()
        .mount(mount)
        .address(checksummed)
        .build()
        .unwrap();
    vaultrs::api::exec_with_result(client, request).await
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
    let checksummed = transaction.from.unwrap_or_default().to_checksum(None);
    let to = if let Some(TxKind::Call(recipient)) = transaction.to {
        Some(format!("0x{:x}", recipient))
    } else {
        // Contract creation transactions have no recipient address
        None
    };
    let request = SignEthereumTransactionRequest::builder()
        .mount(mount)
        .address(checksummed)
        .chain_id(chain_id.to_string())
        .amount(transaction.value.unwrap_or_default().to_string())
        .gas_limit(transaction.gas.unwrap_or(21000))
        .gas_price(transaction.gas_price.unwrap_or_default().to_string())
        .nonce(transaction.nonce.unwrap_or_default())
        .data(transaction.input.input.unwrap_or_default())
        .to(to)
        .build()
        .unwrap();

    vaultrs::api::exec_with_result(client, request).await
}

/// Import a Private Key
/// See [ImportPrivateKeyRequest]
pub async fn import_private_key(
    client: &impl Client,
    mount: &str,
    private_key: B256,
) -> Result<EthereumAccountResponse, ClientError> {
    // The API expects the private key as a hex string without the "0x" prefix
    let private_key_hex = format!("{:x}", private_key);
    let request = ImportPrivateKeyRequest::builder()
        .mount(mount)
        .private_key(private_key_hex)
        .build()
        .unwrap();
    vaultrs::api::exec_with_result(client, request).await
}

/// Sign a message with an Ethereum account.
/// See [SignEthereumRequest]
pub async fn sign(
    client: &impl Client,
    mount: &str,
    address: Address,
    data: &[u8],
) -> Result<EthereumSignResponse, ClientError> {
    let checksummed = address.to_checksum(None);
    let request = EthereumSignRequest::builder()
        .mount(mount)
        .address(checksummed)
        .data(format!("0x{}", hex::encode(data)))
        .build()
        .unwrap();
    vaultrs::api::exec_with_result(client, request).await
}
