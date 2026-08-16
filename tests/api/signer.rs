use std::collections::HashMap;

use quorum_vault_client::api::ethereum::{
    EthereumTransaction, TypedData, TypedDataField, UserOperation,
};
use quorum_vault_client::api::{self, HashFunction, KeyCurve, ethereum};
use vaultrs::client::{VaultClient, VaultClientSettingsBuilder};
use wiremock::matchers::{body_json, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn response(data: serde_json::Value) -> serde_json::Value {
    serde_json::json!({
        "request_id": "request-id",
        "lease_id": "",
        "renewable": false,
        "lease_duration": 0,
        "data": data,
        "wrap_info": null,
        "warnings": null,
        "auth": null
    })
}

async fn client(mock: &MockServer) -> VaultClient {
    VaultClient::new(
        VaultClientSettingsBuilder::default()
            .address(mock.uri())
            .token("token")
            .build()
            .unwrap(),
    )
    .unwrap()
}

#[tokio::test]
async fn creates_key_at_signer_route() {
    let mock = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/signer/keys"))
        .and(body_json(serde_json::json!({
            "id": "secp256k1",
            "curve": "secp256k1",
            "metadata": {"owner": "Alice"}
        })))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(response(serde_json::json!({
                "id": "secp256k1",
                "curve": "secp256k1",
                "metadata": {"owner": "Alice"},
                "public_key": "key"
            }))),
        )
        .mount(&mock)
        .await;

    let key = api::create_key(
        &client(&mock).await,
        "secp256k1",
        KeyCurve::Secp256k1,
        HashMap::from([(String::from("owner"), String::from("Alice"))]),
    )
    .await
    .unwrap();

    assert_eq!(key.id, "secp256k1");
}

#[tokio::test]
async fn signs_hash_and_message_at_signer_routes() {
    let mock = MockServer::start().await;
    let signature = response(serde_json::json!({"signature": "signature"}));
    Mock::given(method("POST"))
        .and(path("/v1/signer/keys/secp256k1/sign/hash"))
        .and(body_json(serde_json::json!({"hash": "abc"})))
        .respond_with(ResponseTemplate::new(200).set_body_json(signature.clone()))
        .mount(&mock)
        .await;
    Mock::given(method("POST"))
        .and(path("/v1/signer/keys/secp256k1/sign/message"))
        .and(body_json(serde_json::json!({
            "message": "68656c6c6f",
            "hash_function": "sha3-256"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(signature))
        .mount(&mock)
        .await;

    let vault = client(&mock).await;
    assert_eq!(
        api::sign_hash(&vault, "secp256k1", "abc")
            .await
            .unwrap()
            .signature,
        "signature"
    );
    assert_eq!(
        api::sign_message(&vault, "secp256k1", "68656c6c6f", HashFunction::Sha3_256)
            .await
            .unwrap()
            .signature,
        "signature"
    );
}

#[tokio::test]
async fn lists_reads_and_deletes_signer_keys() {
    let mock = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/signer/keys"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(response(serde_json::json!({
                "keys": ["secp256k1"]
            }))),
        )
        .mount(&mock)
        .await;
    Mock::given(method("GET"))
        .and(path("/v1/signer/keys/secp256k1"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(response(serde_json::json!({
                "id": "secp256k1",
                "curve": "secp256k1"
            }))),
        )
        .mount(&mock)
        .await;
    Mock::given(method("DELETE"))
        .and(path("/v1/signer/keys/secp256k1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(response(serde_json::Value::Null)))
        .mount(&mock)
        .await;

    let vault = client(&mock).await;
    assert_eq!(api::list_keys(&vault).await.unwrap().keys, ["secp256k1"]);
    assert_eq!(
        api::read_key(&vault, "secp256k1").await.unwrap().id,
        "secp256k1"
    );
    api::delete_key(&vault, "secp256k1").await.unwrap();
}

#[tokio::test]
async fn signs_batch_and_ethereum_payloads() {
    let mock = MockServer::start().await;
    let single_signature = response(serde_json::json!({"signature": "signature"}));
    Mock::given(method("POST"))
        .and(path("/v1/signer/keys/secp256k1/sign/batch"))
        .and(body_json(serde_json::json!({"hashes": ["a", "b"]})))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(response(serde_json::json!({
                "signatures": ["a", "b"]
            }))),
        )
        .mount(&mock)
        .await;
    Mock::given(method("POST"))
        .and(path("/v1/signer/keys/secp256k1/sign/ethereum/transaction"))
        .and(body_json(serde_json::json!({
            "type": "0x2",
            "nonce": "0x0",
            "to": "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045",
            "value": "0x0",
            "gas": "0x5208",
            "maxPriorityFeePerGas": "0x3b9aca00",
            "maxFeePerGas": "0x6fc23ac00",
            "chainId": "0x1"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(single_signature.clone()))
        .mount(&mock)
        .await;
    Mock::given(method("POST"))
        .and(path("/v1/signer/keys/secp256k1/sign/ethereum/typed-data"))
        .and(body_json(serde_json::json!({
            "types": {"Mail": [{"name": "contents", "type": "string"}]},
            "primaryType": "Mail",
            "domain": {"name": "Ether Mail"},
            "message": {"contents": "Hello"}
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(single_signature.clone()))
        .mount(&mock)
        .await;
    Mock::given(method("POST"))
        .and(path(
            "/v1/signer/keys/secp256k1/sign/ethereum/user-operation",
        ))
        .and(body_json(serde_json::json!({
            "userOperation": {
                "sender": "0x1", "nonce": "0x7", "callData": "0xcafebabe",
                "callGasLimit": "0x249f0", "verificationGasLimit": "0x186a0",
                "preVerificationGas": "0xc350", "maxPriorityFeePerGas": "0x64",
                "maxFeePerGas": "0x3e8", "paymaster": "0x2",
                "paymasterVerificationGasLimit": "0xc350",
                "paymasterPostOpGasLimit": "0x7530", "paymasterData": "0xdeadbeef"
            },
            "entryPoint": "0xentry", "entryPointVersion": "0.8", "chainId": "0x1"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(single_signature))
        .mount(&mock)
        .await;

    let vault = client(&mock).await;
    assert_eq!(
        api::sign_batch(&vault, "secp256k1", vec!["a".into(), "b".into()])
            .await
            .unwrap()
            .signatures,
        ["a", "b"]
    );
    ethereum::sign_transaction(
        &vault,
        "secp256k1",
        EthereumTransaction {
            transaction_type: "0x2".into(),
            nonce: "0x0".into(),
            to: Some("0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045".into()),
            value: "0x0".into(),
            gas: "0x5208".into(),
            gas_price: None,
            max_priority_fee_per_gas: Some("0x3b9aca00".into()),
            max_fee_per_gas: Some("0x6fc23ac00".into()),
            chain_id: "0x1".into(),
        },
    )
    .await
    .unwrap();
    ethereum::sign_typed_data(
        &vault,
        "secp256k1",
        TypedData {
            types: HashMap::from([(
                "Mail".into(),
                vec![TypedDataField {
                    name: "contents".into(),
                    field_type: "string".into(),
                }],
            )]),
            primary_type: "Mail".into(),
            domain: serde_json::json!({"name": "Ether Mail"}),
            message: serde_json::json!({"contents": "Hello"}),
        },
    )
    .await
    .unwrap();
    ethereum::sign_user_operation(
        &vault,
        "secp256k1",
        UserOperation {
            sender: "0x1".into(),
            nonce: "0x7".into(),
            call_data: "0xcafebabe".into(),
            call_gas_limit: "0x249f0".into(),
            verification_gas_limit: "0x186a0".into(),
            pre_verification_gas: "0xc350".into(),
            max_priority_fee_per_gas: "0x64".into(),
            max_fee_per_gas: "0x3e8".into(),
            paymaster: Some("0x2".into()),
            paymaster_verification_gas_limit: Some("0xc350".into()),
            paymaster_post_op_gas_limit: Some("0x7530".into()),
            paymaster_data: Some("0xdeadbeef".into()),
        },
        "0xentry",
        "0.8",
        "0x1",
    )
    .await
    .unwrap();
}
