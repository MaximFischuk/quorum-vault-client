use vaultrs::client::Client;
use vaultrs::error::ClientError;

use self::requests::{
    SignEthereumTransactionRequest, SignTypedDataRequest, SignUserOperationRequest,
};
use super::responses::{SignatureResponse, SignedEthereumTransactionResponse};

pub mod requests;

pub use requests::{EthereumTransaction, TypedData, TypedDataField, UserOperation};

/// Signs an Ethereum transaction.
pub async fn sign_transaction(
    client: &impl Client,
    id: &str,
    transaction: EthereumTransaction,
) -> Result<SignedEthereumTransactionResponse, ClientError> {
    let request = SignEthereumTransactionRequest::builder()
        .id(id)
        .transaction(transaction)
        .build()
        .unwrap();
    vaultrs::api::exec_with_result(client, request).await
}

/// Signs EIP-712 typed data.
pub async fn sign_typed_data(
    client: &impl Client,
    id: &str,
    typed_data: TypedData,
) -> Result<SignatureResponse, ClientError> {
    let request = SignTypedDataRequest::builder()
        .id(id)
        .typed_data(typed_data)
        .build()
        .unwrap();
    vaultrs::api::exec_with_result(client, request).await
}

/// Signs an ERC-4337 user operation.
pub async fn sign_user_operation(
    client: &impl Client,
    id: &str,
    user_operation: UserOperation,
    entry_point: &str,
    entry_point_version: &str,
    chain_id: &str,
) -> Result<SignatureResponse, ClientError> {
    let request = SignUserOperationRequest::builder()
        .id(id)
        .user_operation(user_operation)
        .entry_point(entry_point)
        .entry_point_version(entry_point_version)
        .chain_id(chain_id)
        .build()
        .unwrap();
    vaultrs::api::exec_with_result(client, request).await
}
