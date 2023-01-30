use quorum_vault_client::api::KeyCryptoAlgorithm;
use vaultrs::client::{VaultClient, VaultClientSettingsBuilder};

#[tokio::main]
async fn main() {
    // Create a client
    let client = VaultClient::new(
        VaultClientSettingsBuilder::default()
            .address("http://127.0.0.1:8200")
            .token("s.NgpQWnkfYEAxVPC83Bxfa7cy")
            .build()
            .unwrap(),
    )
    .unwrap();

    let key_id = "some-id";

    let new_key = quorum_vault_client::api::create_key(
        &client,
        "quorum",
        key_id,
        KeyCryptoAlgorithm::Secp256k1,
        [("tag".to_string(), "value0".to_string())]
            .into_iter()
            .collect(),
    )
    .await
    .unwrap();
    println!("key: {new_key:?}");

    let keys = quorum_vault_client::api::list_keys(&client, "quorum")
        .await
        .unwrap();
    println!("keys: {keys:?}");

    let read_key = quorum_vault_client::api::read_key(&client, "quorum", key_id)
        .await
        .unwrap();
    println!("read_key: {read_key:?}");

    let signature =
        quorum_vault_client::api::sign(&client, "quorum", key_id, "some-data".as_bytes())
            .await
            .unwrap();
    println!("signature: {signature:?}");

    let updated_tags = quorum_vault_client::api::update_key_tags(
        &client,
        "quorum",
        key_id,
        [("tag".to_string(), "value1".to_string())]
            .into_iter()
            .collect(),
    )
    .await
    .unwrap();
    println!("updated_tags: {updated_tags:?}");

    quorum_vault_client::api::destroy_key(&client, "quorum", "some-id")
        .await
        .unwrap();
    println!("destroyed key: {key_id}");
}
