use quorum_vault_client::api;
use std::str::FromStr;
use vaultrs::client::{VaultClient, VaultClientSettingsBuilder};
use web3::types::{Address, TransactionRequest, U256};
use wiremock::matchers::{body_json, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_create_wallet() {
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
        "request_id": "f69eaa6c-1fe8-faa4-cf0b-6901d69955a8",
        "lease_id": "",
        "renewable": false,
        "lease_duration": 0,
        "data": {
            "address": "0xe4acE2Af74f04689bF0BB86F349747761F31270e",
            "compressed_public_key": "0x031bb8452df3cd3a997a74f0ebbaed4e85136af34532b915eb4cefb0f4feda0a56",
            "namespace": "",
            "public_key": "0x041bb8452df3cd3a997a74f0ebbaed4e85136af34532b915eb4cefb0f4feda0a5697f47f6093b0a1e16de93c5a478771e6e30ac71de08fcb3bc89706630d24aac5"
        },
        "wrap_info": null,
        "warnings": null,
        "auth": null
    });

    Mock::given(method("POST"))
        .and(path("/v1/quorum/ethereum/accounts"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response))
        .mount(&mock)
        .await;

    let wallet = api::create_account(&vault_client, "quorum").await.unwrap();

    assert_eq!(
        format!("{:?}", wallet.address),
        "0xe4acE2Af74f04689bF0BB86F349747761F31270e".to_lowercase()
    );
    assert_eq!(
        wallet.compressed_public_key,
        "0x031bb8452df3cd3a997a74f0ebbaed4e85136af34532b915eb4cefb0f4feda0a56"
    );
    assert_eq!(wallet.public_key, "0x041bb8452df3cd3a997a74f0ebbaed4e85136af34532b915eb4cefb0f4feda0a5697f47f6093b0a1e16de93c5a478771e6e30ac71de08fcb3bc89706630d24aac5");
    assert_eq!(wallet.namespace, "");
}

#[tokio::test]
async fn test_get_list_accounts() {
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
        "request_id": "c189630e-9c1b-23e8-4e91-7e00b4e4135f",
        "lease_id": "",
        "renewable": false,
        "lease_duration": 0,
        "data": {
            "keys": [
                "0xAd38E61dB0D3f8fEF9B4c5DD0C1A9F691cdCcfF5",
                "0xe4acE2Af74f04689bF0BB86F349747761F31270e"
            ]
        },
        "wrap_info": null,
        "warnings": null,
        "auth": null
    });

    Mock::given(method("GET"))
        .and(path("/v1/quorum/ethereum/accounts"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response))
        .mount(&mock)
        .await;

    let accounts = api::list_accounts(&vault_client, "quorum").await.unwrap();

    assert_eq!(accounts.keys.len(), 2);
    assert_eq!(
        format!("{:?}", accounts.keys[0]),
        "0xAd38E61dB0D3f8fEF9B4c5DD0C1A9F691cdCcfF5".to_lowercase()
    );
    assert_eq!(
        format!("{:?}", accounts.keys[1]),
        "0xe4acE2Af74f04689bF0BB86F349747761F31270e".to_lowercase()
    );
}

#[tokio::test]
async fn test_read_account() {
    let mock = MockServer::start().await;
    let vault_client = VaultClient::new(
        VaultClientSettingsBuilder::default()
            .address(mock.uri())
            .token("s.1234567890abcdef")
            .build()
            .unwrap(),
    )
    .unwrap();

    let address = Address::from_str("0xAd38E61dB0D3f8fEF9B4c5DD0C1A9F691cdCcfF5").unwrap();

    let response = serde_json::json!({
        "request_id": "09096b87-393b-00a0-a8ec-540d6023850d",
        "lease_id": "",
        "renewable": false,
        "lease_duration": 0,
        "data": {
            "address": "0xAd38E61dB0D3f8fEF9B4c5DD0C1A9F691cdCcfF5",
            "compressed_public_key": "0x03e4e5a8a5a1c909b70e8a9b9ce8cb02dc7e2783580d6e5babb989cb595e407704",
            "namespace": "",
            "public_key": "0x04e4e5a8a5a1c909b70e8a9b9ce8cb02dc7e2783580d6e5babb989cb595e4077040c1f6b6c38ac7ec3b74880aa578c6151e835e6265db6ce4d1e45a5c31855a689"
        },
        "wrap_info": null,
        "warnings": null,
        "auth": null
    });

    Mock::given(method("GET"))
        .and(path(
            "/v1/quorum/ethereum/accounts/0xAd38E61dB0D3f8fEF9B4c5DD0C1A9F691cdCcfF5",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response))
        .mount(&mock)
        .await;

    let account = api::read_account(&vault_client, "quorum", address)
        .await
        .unwrap();

    assert_eq!(
        format!("{:?}", account.address),
        "0xAd38E61dB0D3f8fEF9B4c5DD0C1A9F691cdCcfF5".to_lowercase()
    );
    assert_eq!(
        account.compressed_public_key,
        "0x03e4e5a8a5a1c909b70e8a9b9ce8cb02dc7e2783580d6e5babb989cb595e407704"
    );
    assert_eq!(account.public_key, "0x04e4e5a8a5a1c909b70e8a9b9ce8cb02dc7e2783580d6e5babb989cb595e4077040c1f6b6c38ac7ec3b74880aa578c6151e835e6265db6ce4d1e45a5c31855a689");
    assert_eq!(account.namespace, "");
}

#[tokio::test]
async fn test_sign_transaction() {
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
      "chain_id": "1",
      "amount": "1000000000000000000",
      "data": "0x",
      "gas_limit": 21000,
      "gas_price": "1",
      "nonce": 0,
      "to": "0x1dabe0acaaa4d1f81b9b43eaf51c8439378231a0",
    });
    let response = serde_json::json!({
        "request_id": "5cede0bc-f7ce-d7a7-cba5-2427cee48bd5",
        "lease_id": "",
        "renewable": false,
        "lease_duration": 0,
        "data": {
            "signature": "0x3f3c03151a9451832d7b3abacec63cee23e4f697690db03e980ce5b02594a6e6657e236309cbcaa0de351162d69422aa3ebaf2349a8c5e612bd971d9de18be8501"
        },
        "wrap_info": null,
        "warnings": null,
        "auth": null
    });

    Mock::given(method("POST"))
        .and(path("/v1/quorum/ethereum/accounts/0xAd38E61dB0D3f8fEF9B4c5DD0C1A9F691cdCcfF5/sign-transaction"))
        .and(body_json(expected_request))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response))
        .mount(&mock)
        .await;

    let address = Address::from_str("0xAd38E61dB0D3f8fEF9B4c5DD0C1A9F691cdCcfF5").unwrap();
    let to = Address::from_str("0x1daBe0aCaAA4D1F81b9b43Eaf51C8439378231a0").unwrap();

    let mut tx: TransactionRequest = TransactionRequest::builder()
        .from(address)
        .to(to)
        .value(U256::from_dec_str("1000000000000000000").unwrap())
        .gas(U256::from(21000))
        .nonce(U256::from(0))
        .build();

    tx.gas_price = Some(U256::from(1));

    let signature = api::sign_transaction(&vault_client, "quorum", 1, tx)
        .await
        .unwrap();

    assert_eq!(signature.signature, "0x3f3c03151a9451832d7b3abacec63cee23e4f697690db03e980ce5b02594a6e6657e236309cbcaa0de351162d69422aa3ebaf2349a8c5e612bd971d9de18be8501");
}

#[tokio::test]
async fn test_import_private_key() {
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
      "private_key": "0a1232595b77534d99364bfde13383accbcb40775967a7eacd15d355c96288a5"
    });
    let response = serde_json::json!({
        "request_id": "a7e5cd28-2867-86d5-7f82-73650ec68950",
        "lease_id": "",
        "renewable": false,
        "lease_duration": 0,
        "data": {
            "address": "0xeCB96104c306DF32Aed607EF0B8a44cC94BE782F",
            "compressed_public_key": "0x026b5ae5ec570abb9c4c50746d08fb63c911641170581b07f5f531a993b8b6cbec",
            "namespace": "",
            "public_key": "0x046b5ae5ec570abb9c4c50746d08fb63c911641170581b07f5f531a993b8b6cbeced5f8e3de3f4c7416a7661ed2c7eef8fea416c62df47ec43896af26086b87594"
        },
        "wrap_info": null,
        "warnings": null,
        "auth": null
    });

    Mock::given(method("POST"))
        .and(path("/v1/quorum/ethereum/accounts/import"))
        .and(body_json(expected_request))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response))
        .mount(&mock)
        .await;

    let wallet = api::import_private_key(
        &vault_client,
        "quorum",
        "0a1232595b77534d99364bfde13383accbcb40775967a7eacd15d355c96288a5",
    )
    .await
    .unwrap();

    assert_eq!(
        format!("{:?}", wallet.address),
        "0xeCB96104c306DF32Aed607EF0B8a44cC94BE782F".to_lowercase()
    );
    assert_eq!(
        wallet.compressed_public_key,
        "0x026b5ae5ec570abb9c4c50746d08fb63c911641170581b07f5f531a993b8b6cbec"
    );
    assert_eq!(wallet.public_key, "0x046b5ae5ec570abb9c4c50746d08fb63c911641170581b07f5f531a993b8b6cbeced5f8e3de3f4c7416a7661ed2c7eef8fea416c62df47ec43896af26086b87594");
    assert_eq!(wallet.namespace, "");
}
