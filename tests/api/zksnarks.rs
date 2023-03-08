use quorum_vault_client::api;
use vaultrs::client::{VaultClient, VaultClientSettingsBuilder};
use web3::signing::keccak256;
use wiremock::matchers::{body_json, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_create_zksnarks_account() {
    let mock = MockServer::start().await;
    let vault_client = VaultClient::new(
        VaultClientSettingsBuilder::default()
            .address(mock.uri())
            .token("s.1234567890abcdef")
            .build()
            .unwrap(),
    )
    .unwrap();

    let response = serde_json::json!({
        "request_id": "e81af2c4-4e4c-a640-0f8f-99ce3f7d486a",
        "lease_id": "",
        "renewable": false,
        "lease_duration": 0,
        "data": {
            "curve": "babyjubjub",
            "namespace": "",
            "public_key": "0x7e8249b895434a1b02aade22033b887620ab5e756aa106d415ff33ace9048626",
            "signing_algorithm": "eddsa"
        },
        "wrap_info": null,
        "warnings": null,
        "auth": null
    });

    Mock::given(method("POST"))
        .and(path("/v1/quorum/zk-snarks/accounts"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response))
        .mount(&mock)
        .await;

    let account = api::create_zksnarks_account(&vault_client, "quorum")
        .await
        .unwrap();

    assert_eq!(account.curve, "babyjubjub");
    assert_eq!(account.namespace, "");
    assert_eq!(
        account.public_key,
        "0x7e8249b895434a1b02aade22033b887620ab5e756aa106d415ff33ace9048626"
    );
    assert_eq!(account.signing_algorithm, "eddsa");
}

#[tokio::test]
async fn test_list_zksnarks_account() {
    let mock = MockServer::start().await;
    let vault_client = VaultClient::new(
        VaultClientSettingsBuilder::default()
            .address(mock.uri())
            .token("s.1234567890abcdef")
            .build()
            .unwrap(),
    )
    .unwrap();

    let response = serde_json::json!({
        "request_id": "2bd76aaf-405e-0330-2202-ea8361dae53a",
        "lease_id": "",
        "renewable": false,
        "lease_duration": 0,
        "data": {
            "keys": [
                "0x7e8249b895434a1b02aade22033b887620ab5e756aa106d415ff33ace9048626"
            ]
        },
        "wrap_info": null,
        "warnings": null,
        "auth": null
    });

    Mock::given(method("GET"))
        .and(path("/v1/quorum/zk-snarks/accounts"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response))
        .mount(&mock)
        .await;

    let accounts = api::list_zksnarks_accounts(&vault_client, "quorum")
        .await
        .unwrap();

    assert_eq!(accounts.keys.len(), 1);
    assert_eq!(
        accounts.keys[0],
        "0x7e8249b895434a1b02aade22033b887620ab5e756aa106d415ff33ace9048626"
    );
}

#[tokio::test]
async fn test_read_zksnarks_account() {
    let mock = MockServer::start().await;
    let vault_client = VaultClient::new(
        VaultClientSettingsBuilder::default()
            .address(mock.uri())
            .token("s.1234567890abcdef")
            .build()
            .unwrap(),
    )
    .unwrap();

    let response = serde_json::json!({
        "request_id": "e81af2c4-4e4c-a640-0f8f-99ce3f7d486a",
        "lease_id": "",
        "renewable": false,
        "lease_duration": 0,
        "data": {
            "curve": "babyjubjub",
            "namespace": "",
            "public_key": "0x7e8249b895434a1b02aade22033b887620ab5e756aa106d415ff33ace9048626",
            "signing_algorithm": "eddsa"
        },
        "wrap_info": null,
        "warnings": null,
        "auth": null
    });

    Mock::given(method("GET"))
        .and(path("/v1/quorum/zk-snarks/accounts/0x7e8249b895434a1b02aade22033b887620ab5e756aa106d415ff33ace9048626"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response))
        .mount(&mock)
        .await;

    let account = api::read_zksnarks_account(
        &vault_client,
        "quorum",
        "0x7e8249b895434a1b02aade22033b887620ab5e756aa106d415ff33ace9048626",
    )
    .await
    .unwrap();

    assert_eq!(account.curve, "babyjubjub");
    assert_eq!(account.namespace, "");
    assert_eq!(
        account.public_key,
        "0x7e8249b895434a1b02aade22033b887620ab5e756aa106d415ff33ace9048626"
    );
    assert_eq!(account.signing_algorithm, "eddsa");
}

#[tokio::test]
async fn test_sign_message() {
    let mock = MockServer::start().await;
    let vault_client = VaultClient::new(
        VaultClientSettingsBuilder::default()
            .address(mock.uri())
            .token("s.1234567890abcdef")
            .build()
            .unwrap(),
    )
    .unwrap();

    let data = "Hello, world!";

    let expected_request = serde_json::json!({
        "data": "0xb6e16d27ac5ab427a7f68900ac5559ce272dc6c37c82b3e052246c82244c50e4"
    });

    let response = serde_json::json!({
        "request_id": "e81af2c4-4e4c-a640-0f8f-99ce3f7d486a",
        "lease_id": "",
        "renewable": false,
        "lease_duration": 0,
        "data": {
            "signature": "0xac34541ff103beac043f2525d756c9a5f4288be4910c33f49c4fcea69b766ca6011b28e6ad62a1a3eddf2cc08ca7265553c175ffa60982616fa4facaf5f87d4a"
        },
        "wrap_info": null,
        "warnings": null,
        "auth": null
    });

    Mock::given(method("POST"))
        .and(path("/v1/quorum/zk-snarks/accounts/0x7e8249b895434a1b02aade22033b887620ab5e756aa106d415ff33ace9048626/sign"))
        .and(body_json(&expected_request))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response))
        .mount(&mock)
        .await;

    let signature = api::zksnarks_sign(
        &vault_client,
        "quorum",
        "0x7e8249b895434a1b02aade22033b887620ab5e756aa106d415ff33ace9048626",
        data.as_bytes(),
    )
    .await
    .unwrap();

    assert_eq!(signature.signature, "0xac34541ff103beac043f2525d756c9a5f4288be4910c33f49c4fcea69b766ca6011b28e6ad62a1a3eddf2cc08ca7265553c175ffa60982616fa4facaf5f87d4a");
}

#[tokio::test]
async fn test_sign_hash() {
    let mock = MockServer::start().await;
    let vault_client = VaultClient::new(
        VaultClientSettingsBuilder::default()
            .address(mock.uri())
            .token("s.1234567890abcdef")
            .build()
            .unwrap(),
    )
    .unwrap();

    let data = "Hello, world!";
    let hash = keccak256(data.as_bytes());

    let expected_request = serde_json::json!({
        "data": "0xb6e16d27ac5ab427a7f68900ac5559ce272dc6c37c82b3e052246c82244c50e4"
    });

    let response = serde_json::json!({
        "request_id": "e81af2c4-4e4c-a640-0f8f-99ce3f7d486a",
        "lease_id": "",
        "renewable": false,
        "lease_duration": 0,
        "data": {
            "signature": "0xac34541ff103beac043f2525d756c9a5f4288be4910c33f49c4fcea69b766ca6011b28e6ad62a1a3eddf2cc08ca7265553c175ffa60982616fa4facaf5f87d4a"
        },
        "wrap_info": null,
        "warnings": null,
        "auth": null
    });

    Mock::given(method("POST"))
        .and(path("/v1/quorum/zk-snarks/accounts/0x7e8249b895434a1b02aade22033b887620ab5e756aa106d415ff33ace9048626/sign"))
        .and(body_json(&expected_request))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response))
        .mount(&mock)
        .await;

    let signature = api::zksnarks_sign_hash(
        &vault_client,
        "quorum",
        "0x7e8249b895434a1b02aade22033b887620ab5e756aa106d415ff33ace9048626",
        hash,
    )
    .await
    .unwrap();

    assert_eq!(signature.signature, "0xac34541ff103beac043f2525d756c9a5f4288be4910c33f49c4fcea69b766ca6011b28e6ad62a1a3eddf2cc08ca7265553c175ffa60982616fa4facaf5f87d4a");
}
