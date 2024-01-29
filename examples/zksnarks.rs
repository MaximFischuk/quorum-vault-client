use vaultrs::client::{VaultClient, VaultClientSettingsBuilder};

#[tokio::main]
async fn main() {
    // Create a client
    let client = VaultClient::new(
        VaultClientSettingsBuilder::default()
            .address("http://localhost:8200")
            .token("root")
            .build()
            .unwrap(),
    )
    .unwrap();

    // Create a new account
    let account = quorum_vault_client::api::zksnarks::create_zksnarks_account(&client, "quorum")
        .await
        .unwrap();
    println!("account: {:?}", account);

    // Read the account
    let account = quorum_vault_client::api::zksnarks::read_zksnarks_account(
        &client,
        "quorum",
        &account.public_key,
    )
    .await
    .unwrap();
    println!("account: {:?}", account);

    // List the accounts
    let accounts = quorum_vault_client::api::zksnarks::list_zksnarks_accounts(&client, "quorum")
        .await
        .unwrap();
    println!("accounts: {:?}", accounts);

    // Sign a message
    let signature = quorum_vault_client::api::zksnarks::zksnarks_sign(
        &client,
        "quorum",
        &account.public_key,
        "some-data".as_bytes(),
    )
    .await
    .unwrap();
    println!("signature: {:?}", signature);
}
