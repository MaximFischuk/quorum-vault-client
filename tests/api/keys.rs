use quorum_vault_client::api;
use quorum_vault_client::api::KeyCryptoAlgorithm;
use vaultrs::client::{VaultClient, VaultClientSettingsBuilder};
use wiremock::matchers::{body_json, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_create_key() {
    let mock = MockServer::start().await;
    let vault_client = VaultClient::new(
        VaultClientSettingsBuilder::default()
            .address(mock.uri())
            .token("s.1234567890abcdef")
            .build()
            .unwrap(),
    )
    .unwrap();

    let expected_request = serde_json::json!({
        "signing_algorithm": "ecdsa",
        "tags": {
            "env": "dev",
            "kind": "wallet"
        },
        "id": "dd4b594d-4b89-480d-a8a8-01ed7e1f0140",
        "curve": "secp256k1"
    });

    let response = serde_json::json!({
        "request_id": "b5efba4e-9ed6-5e53-66ab-c072a078e6f5",
        "lease_id": "",
        "renewable": false,
        "lease_duration": 0,
        "data": {
            "created_at": "2023-01-28T13:33:28.583408531Z",
            "curve": "secp256k1",
            "id": "dd4b594d-4b89-480d-a8a8-01ed7e1f0140",
            "namespace": "",
            "public_key": "BMDV2nRbw6iFuqs0RRF9cdhgvvHDoxmA4MPN0jomXTGhQIhPlCLMsnWFHaoeYihRUNnQ01CAftQUkAMol8G5SuU=",
            "signing_algorithm": "ecdsa",
            "tags": {
                "env": "dev",
                "kind": "wallet"
            },
            "updated_at": "2023-01-28T13:33:28.583408531Z",
            "version": 1
        },
        "wrap_info": null,
        "warnings": null,
        "auth": null
    });

    Mock::given(method("POST"))
        .and(path("/v1/quorum/keys"))
        .and(body_json(&expected_request))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response))
        .mount(&mock)
        .await;

    let key = api::create_key(
        &vault_client,
        "quorum",
        "dd4b594d-4b89-480d-a8a8-01ed7e1f0140",
        KeyCryptoAlgorithm::Secp256k1,
        vec![
            ("env".to_string(), "dev".to_string()),
            ("kind".to_string(), "wallet".to_string()),
        ]
        .into_iter()
        .collect(),
    )
    .await
    .unwrap();

    assert_eq!(key.id, "dd4b594d-4b89-480d-a8a8-01ed7e1f0140");
    assert_eq!(key.curve, "secp256k1");
    assert_eq!(key.signing_algorithm, "ecdsa");
    assert_eq!(
        key.tags,
        vec![
            ("env".to_string(), "dev".to_string()),
            ("kind".to_string(), "wallet".to_string())
        ]
        .into_iter()
        .collect()
    );
    assert_eq!(key.version, 1);
}

#[tokio::test]
async fn test_read_key() {
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
        "request_id": "9e4325b4-9d13-5e1c-c8f8-4c568b1728f0",
        "lease_id": "",
        "renewable": false,
        "lease_duration": 0,
        "data": {
            "created_at": "2023-01-28T13:33:28.583408531Z",
            "curve": "secp256k1",
            "id": "dd4b594d-4b89-480d-a8a8-01ed7e1f0140",
            "namespace": "",
            "public_key": "BMDV2nRbw6iFuqs0RRF9cdhgvvHDoxmA4MPN0jomXTGhQIhPlCLMsnWFHaoeYihRUNnQ01CAftQUkAMol8G5SuU=",
            "signing_algorithm": "ecdsa",
            "tags": {
                "env": "dev",
                "kind": "wallet"
            },
            "updated_at": "2023-01-28T13:33:28.583408531Z",
            "version": 1
        },
        "wrap_info": null,
        "warnings": null,
        "auth": null
    });

    Mock::given(method("GET"))
        .and(path("/v1/quorum/keys/dd4b594d-4b89-480d-a8a8-01ed7e1f0140"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response))
        .mount(&mock)
        .await;

    let key = api::read_key(
        &vault_client,
        "quorum",
        "dd4b594d-4b89-480d-a8a8-01ed7e1f0140",
    )
    .await
    .unwrap();

    assert_eq!(key.id, "dd4b594d-4b89-480d-a8a8-01ed7e1f0140");
    assert_eq!(key.curve, "secp256k1");
    assert_eq!(key.signing_algorithm, "ecdsa");
    assert_eq!(
        key.tags,
        vec![
            ("env".to_string(), "dev".to_string()),
            ("kind".to_string(), "wallet".to_string())
        ]
        .into_iter()
        .collect()
    );
    assert_eq!(key.version, 1);
}

#[tokio::test]
async fn test_get_list_keys() {
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
        "request_id": "e3d064d4-8a4d-e7d3-475b-8aa00f43e75a",
        "lease_id": "",
        "renewable": false,
        "lease_duration": 0,
        "data": {
            "keys": [
                "dd4b594d-4b89-480d-a8a8-01ed7e1f0140"
            ]
        },
        "wrap_info": null,
        "warnings": null,
        "auth": null
    });

    Mock::given(method("GET"))
        .and(path("/v1/quorum/keys"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response))
        .mount(&mock)
        .await;

    let keys = api::list_keys(&vault_client, "quorum").await.unwrap();

    assert_eq!(keys.keys, vec!["dd4b594d-4b89-480d-a8a8-01ed7e1f0140"]);
}

#[tokio::test]
async fn test_sign_data() {
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

    let expected_request: serde_json::Value = serde_json::json!({
        "data": "tuFtJ6xatCen9okArFVZzictxsN8grPgUiRsgiRMUOQ="
    });
    let response = serde_json::json!({
        "request_id": "73baca4c-5690-a384-2105-751e4e33f0da",
        "lease_id": "",
        "renewable": false,
        "lease_duration": 0,
        "data": {
            "signature": "HgBoSOuweiQUHuXpTjqaSv8yoGUDh37GnMJg9ZyOeTRrjrE9xetYSq-Onjej_kdswHj8FnNRxRhqpYt8jrX71w=="
        },
        "wrap_info": null,
        "warnings": null,
        "auth": null
    });

    Mock::given(method("POST"))
        .and(path(
            "/v1/quorum/keys/dd4b594d-4b89-480d-a8a8-01ed7e1f0140/sign",
        ))
        .and(body_json(&expected_request))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response))
        .mount(&mock)
        .await;

    let signature = api::sign(
        &vault_client,
        "quorum",
        "dd4b594d-4b89-480d-a8a8-01ed7e1f0140",
        data.as_bytes(),
    )
    .await
    .unwrap();

    assert_eq!(
        signature.signature,
        "HgBoSOuweiQUHuXpTjqaSv8yoGUDh37GnMJg9ZyOeTRrjrE9xetYSq-Onjej_kdswHj8FnNRxRhqpYt8jrX71w=="
    );
}

#[tokio::test]
async fn test_update_key_tags() {
    let mock = MockServer::start().await;
    let vault_client = VaultClient::new(
        VaultClientSettingsBuilder::default()
            .address(mock.uri())
            .token("s.1234567890abcdef")
            .build()
            .unwrap(),
    )
    .unwrap();

    let expected_request: serde_json::Value = serde_json::json!({
        "tags": {
            "env": "dev",
            "kind": "wallet"
        }
    });
    let response = serde_json::json!({
        "request_id": "39fdefc5-1f30-b5fe-08ba-02dd75f96daa",
        "lease_id": "",
        "renewable": false,
        "lease_duration": 0,
        "data": {
            "created_at": "2023-01-28T13:33:28.583408531Z",
            "curve": "secp256k1",
            "id": "dd4b594d-4b89-480d-a8a8-01ed7e1f0140",
            "namespace": "",
            "public_key": "BMDV2nRbw6iFuqs0RRF9cdhgvvHDoxmA4MPN0jomXTGhQIhPlCLMsnWFHaoeYihRUNnQ01CAftQUkAMol8G5SuU=",
            "signing_algorithm": "ecdsa",
            "tags": {
                "env": "dev",
                "kind": "wallet"
            },
            "updated_at": "2023-01-28T13:33:28.583408531Z",
            "version": 1
        },
        "wrap_info": null,
        "warnings": null,
        "auth": null
    });

    Mock::given(method("POST"))
        .and(path("/v1/quorum/keys/dd4b594d-4b89-480d-a8a8-01ed7e1f0140"))
        .and(body_json(&expected_request))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response))
        .mount(&mock)
        .await;

    let key = api::update_key_tags(
        &vault_client,
        "quorum",
        "dd4b594d-4b89-480d-a8a8-01ed7e1f0140",
        [
            ("env".to_string(), "dev".to_string()),
            ("kind".to_string(), "wallet".to_string()),
        ]
        .into_iter()
        .collect(),
    )
    .await
    .unwrap();

    assert_eq!(key.tags.get("env").unwrap(), "dev");
    assert_eq!(key.tags.get("kind").unwrap(), "wallet");
}

#[tokio::test]
async fn test_destroy_key() {
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
        "request_id": "39fdefc5-1f30-b5fe-08ba-02dd75f96daa",
        "lease_id": "",
        "renewable": false,
        "lease_duration": 0,
        "data": null,
        "wrap_info": null,
        "warnings": null,
        "auth": null
    });

    Mock::given(method("DELETE"))
        .and(path(
            "/v1/quorum/keys/dd4b594d-4b89-480d-a8a8-01ed7e1f0140/destroy",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response))
        .mount(&mock)
        .await;

    api::destroy_key(
        &vault_client,
        "quorum",
        "dd4b594d-4b89-480d-a8a8-01ed7e1f0140",
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn test_import_key() {
    let mock = MockServer::start().await;
    let vault_client = VaultClient::new(
        VaultClientSettingsBuilder::default()
            .address(mock.uri())
            .token("s.1234567890abcdef")
            .build()
            .unwrap(),
    )
    .unwrap();

    let expected_request = serde_json::json!({
      "signing_algorithm": "ecdsa",
      "tags": {
          "env": "dev",
          "kind": "wallet"
      },
      "private_key": "1s1iv38t1CBEGJOHo2ah7yrrhFoUZKiiU4aUq_59B3A=",
      "id": "dd4b594d-4b89-480d-a8a8-01ed7e1f0140",
      "curve": "secp256k1"
    });
    let response = serde_json::json!({
        "request_id": "39fdefc5-1f30-b5fe-08ba-02dd75f96daa",
        "lease_id": "",
        "renewable": false,
        "lease_duration": 0,
        "data": {
            "created_at": "2023-01-28T13:33:28.583408531Z",
            "curve": "secp256k1",
            "id": "dd4b594d-4b89-480d-a8a8-01ed7e1f0140",
            "namespace": "",
            "public_key": "BMDV2nRbw6iFuqs0RRF9cdhgvvHDoxmA4MPN0jomXTGhQIhPlCLMsnWFHaoeYihRUNnQ01CAftQUkAMol8G5SuU=",
            "signing_algorithm": "ecdsa",
            "tags": {
                "env": "dev",
                "kind": "wallet"
            },
            "updated_at": "2023-01-28T13:33:28.583408531Z",
            "version": 1
        },
        "wrap_info": null,
        "warnings": null,
        "auth": null
    });

    Mock::given(method("POST"))
        .and(path("/v1/quorum/keys/import"))
        .and(body_json(&expected_request))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response))
        .mount(&mock)
        .await;

    let key = api::import_key(
        &vault_client,
        "quorum",
        "dd4b594d-4b89-480d-a8a8-01ed7e1f0140",
        KeyCryptoAlgorithm::Secp256k1,
        [
            ("env".to_string(), "dev".to_string()),
            ("kind".to_string(), "wallet".to_string()),
        ]
        .into_iter()
        .collect(),
        "1s1iv38t1CBEGJOHo2ah7yrrhFoUZKiiU4aUq_59B3A=",
    )
    .await
    .unwrap();

    assert_eq!(key.tags.get("env").unwrap(), "dev");
    assert_eq!(key.tags.get("kind").unwrap(), "wallet");
    assert_eq!(key.id, "dd4b594d-4b89-480d-a8a8-01ed7e1f0140");
    assert_eq!(key.curve, "secp256k1");
    assert_eq!(key.signing_algorithm, "ecdsa");
    assert_eq!(key.version, 1);
    assert_eq!(key.public_key, "BMDV2nRbw6iFuqs0RRF9cdhgvvHDoxmA4MPN0jomXTGhQIhPlCLMsnWFHaoeYihRUNnQ01CAftQUkAMol8G5SuU=");
}
