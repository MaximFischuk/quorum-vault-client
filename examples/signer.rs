use std::collections::HashMap;

use quorum_vault_client::{
    VaultClient, VaultClientSettingsBuilder,
    api::{self, KeyCurve},
};

#[tokio::main]
async fn main() {
    let client = VaultClient::new(
        VaultClientSettingsBuilder::default()
            .address("http://127.0.0.1:8200")
            .token("root")
            .build()
            .unwrap(),
    )
    .unwrap();

    let key = api::create_key(
        &client,
        "secp256k1",
        KeyCurve::Secp256k1,
        HashMap::from([(String::from("owner"), String::from("Alice"))]),
    )
    .await
    .unwrap();

    println!("key: {key:?}");
}
