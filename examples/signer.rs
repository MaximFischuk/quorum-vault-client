use std::collections::HashMap;

use quorum_vault_client::{
    VaultClient, VaultClientSettingsBuilder,
    api::{
        self, EthereumTransaction, HashFunction, KeyCurve, TypedData, TypedDataField, UserOperation,
    },
};

const VAULT_ADDRESS: &str = "http://127.0.0.1:8200";
const VAULT_TOKEN: &str = "DevVaultToken";
const MOUNT: &str = "signer";
const KEY_ID: &str = "quorum-client-example-secp256k1";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = VaultClient::new(
        VaultClientSettingsBuilder::default()
            .address(VAULT_ADDRESS)
            .token(VAULT_TOKEN)
            .build()?,
    )?;

    let created_key = !api::list_keys(&client, MOUNT)
        .await?
        .keys
        .iter()
        .any(|key_id| key_id == KEY_ID);
    let key = if created_key {
        api::create_key(
            &client,
            MOUNT,
            KEY_ID,
            KeyCurve::Secp256k1,
            HashMap::from([(String::from("owner"), String::from("Alice"))]),
        )
        .await?
    } else {
        api::read_key(&client, MOUNT, KEY_ID).await?
    };

    println!("key: {key:?}");
    println!("keys: {:?}", api::list_keys(&client, MOUNT).await?.keys);
    println!("key: {:?}", api::read_key(&client, MOUNT, KEY_ID).await?);

    let hash = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    println!(
        "hash signature: {:?}",
        api::sign_hash(&client, MOUNT, KEY_ID, hash).await?
    );
    println!(
        "batch signatures: {:?}",
        api::sign_batch(
            &client,
            MOUNT,
            KEY_ID,
            vec![hash.into(), hash.replace('a', "b")],
        )
        .await?
    );
    println!(
        "message signature: {:?}",
        api::sign_message(
            &client,
            MOUNT,
            KEY_ID,
            "68656c6c6f",
            HashFunction::Keccak256,
        )
        .await?
    );

    println!(
        "Ethereum transaction signature: {:?}",
        api::sign_ethereum_transaction(
            &client,
            MOUNT,
            KEY_ID,
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
        .await?
    );

    println!(
        "EIP-712 signature: {:?}",
        api::sign_typed_data(
            &client,
            MOUNT,
            KEY_ID,
            TypedData {
                types: HashMap::from([
                    (
                        "EIP712Domain".into(),
                        vec![TypedDataField {
                            name: "name".into(),
                            field_type: "string".into(),
                        }],
                    ),
                    (
                        "Mail".into(),
                        vec![TypedDataField {
                            name: "contents".into(),
                            field_type: "string".into(),
                        }],
                    ),
                ]),
                primary_type: "Mail".into(),
                domain: serde_json::json!({"name": "Ether Mail"}),
                message: serde_json::json!({"contents": "Hello"}),
            },
        )
        .await?
    );

    println!(
        "ERC-4337 signature: {:?}",
        api::sign_user_operation(
            &client,
            MOUNT,
            KEY_ID,
            UserOperation {
                sender: "0x0000000000000000000000000000000000000001".into(),
                nonce: "0x7".into(),
                call_data: "0xcafebabe".into(),
                call_gas_limit: "0x249f0".into(),
                verification_gas_limit: "0x186a0".into(),
                pre_verification_gas: "0xc350".into(),
                max_priority_fee_per_gas: "0x64".into(),
                max_fee_per_gas: "0x3e8".into(),
                paymaster: None,
                paymaster_verification_gas_limit: None,
                paymaster_post_op_gas_limit: None,
                paymaster_data: None,
            },
            "0x0000000071727De22E5E9d8bAF0edAc6f37da032",
            "0.8",
            "0x1",
        )
        .await?
    );

    if created_key {
        api::delete_key(&client, MOUNT, KEY_ID).await?;
        println!("deleted key: {KEY_ID}");
    } else {
        println!("kept existing key: {KEY_ID}");
    }

    Ok(())
}
