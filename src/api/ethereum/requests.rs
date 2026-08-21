use std::collections::HashMap;

use derive_builder::Builder;
use rustify_derive::Endpoint;
use serde::Serialize;

use crate::api::responses::{SignatureResponse, SignedEthereumTransactionResponse};

/// Ethereum transaction fields accepted by signer plugin.
#[derive(Clone, Debug, Serialize)]
pub struct EthereumTransaction {
    #[serde(rename = "type")]
    pub transaction_type: String,
    pub nonce: String,
    pub to: Option<String>,
    pub value: String,
    pub gas: String,
    #[serde(rename = "gasPrice", skip_serializing_if = "Option::is_none")]
    pub gas_price: Option<String>,
    #[serde(
        rename = "maxPriorityFeePerGas",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_priority_fee_per_gas: Option<String>,
    #[serde(rename = "maxFeePerGas", skip_serializing_if = "Option::is_none")]
    pub max_fee_per_gas: Option<String>,
    #[serde(rename = "chainId")]
    pub chain_id: String,
}

/// ERC-4337 user operation fields accepted by signer plugin.
#[derive(Clone, Debug, Serialize)]
pub struct UserOperation {
    pub sender: String,
    pub nonce: String,
    #[serde(rename = "callData")]
    pub call_data: String,
    #[serde(rename = "callGasLimit")]
    pub call_gas_limit: String,
    #[serde(rename = "verificationGasLimit")]
    pub verification_gas_limit: String,
    #[serde(rename = "preVerificationGas")]
    pub pre_verification_gas: String,
    #[serde(rename = "maxPriorityFeePerGas")]
    pub max_priority_fee_per_gas: String,
    #[serde(rename = "maxFeePerGas")]
    pub max_fee_per_gas: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paymaster: Option<String>,
    #[serde(
        rename = "paymasterVerificationGasLimit",
        skip_serializing_if = "Option::is_none"
    )]
    pub paymaster_verification_gas_limit: Option<String>,
    #[serde(
        rename = "paymasterPostOpGasLimit",
        skip_serializing_if = "Option::is_none"
    )]
    pub paymaster_post_op_gas_limit: Option<String>,
    #[serde(rename = "paymasterData", skip_serializing_if = "Option::is_none")]
    pub paymaster_data: Option<String>,
}

/// EIP-712 field definition.
#[derive(Clone, Debug, Serialize)]
pub struct TypedDataField {
    pub name: String,
    #[serde(rename = "type")]
    pub field_type: String,
}

/// EIP-712 typed data accepted by signer plugin.
#[derive(Clone, Debug, Serialize)]
pub struct TypedData {
    pub types: HashMap<String, Vec<TypedDataField>>,
    #[serde(rename = "primaryType")]
    pub primary_type: String,
    pub domain: serde_json::Value,
    pub message: serde_json::Value,
}

#[derive(Builder, Debug, Endpoint, Serialize)]
#[endpoint(
    path = "signer/keys/{self.id}/sign/ethereum/transaction",
    method = "POST",
    response = "SignedEthereumTransactionResponse",
    builder = "true"
)]
#[builder(setter(into))]
pub struct SignEthereumTransactionRequest {
    #[endpoint(skip)]
    pub id: String,
    #[endpoint(body)]
    #[serde(flatten)]
    pub transaction: EthereumTransaction,
}

#[derive(Builder, Debug, Endpoint, Serialize)]
#[endpoint(
    path = "signer/keys/{self.id}/sign/ethereum/typed-data",
    method = "POST",
    response = "SignatureResponse",
    builder = "true"
)]
#[builder(setter(into))]
pub struct SignTypedDataRequest {
    #[endpoint(skip)]
    pub id: String,
    #[endpoint(body)]
    #[serde(flatten)]
    pub typed_data: TypedData,
}

#[derive(Builder, Debug, Endpoint, Serialize)]
#[endpoint(
    path = "signer/keys/{self.id}/sign/ethereum/user-operation",
    method = "POST",
    response = "SignatureResponse",
    builder = "true"
)]
#[builder(setter(into))]
pub struct SignUserOperationRequest {
    #[endpoint(skip)]
    pub id: String,
    #[serde(rename = "userOperation")]
    pub user_operation: UserOperation,
    #[serde(rename = "entryPoint")]
    pub entry_point: String,
    #[serde(rename = "entryPointVersion")]
    pub entry_point_version: String,
    #[serde(rename = "chainId")]
    pub chain_id: String,
}
