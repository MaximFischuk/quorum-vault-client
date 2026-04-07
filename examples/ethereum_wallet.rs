use alloy_primitives::U256;
use alloy_rpc_types_eth::TransactionRequest;
use std::str::FromStr;
use vaultrs::client::{VaultClient, VaultClientSettingsBuilder};

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

    let created_account = quorum_vault_client::api::ethereum::create_account(&client, "quorum")
        .await
        .unwrap();
    println!("result: {:?}", created_account);

    let addresses = quorum_vault_client::api::ethereum::list_accounts(&client, "quorum")
        .await
        .unwrap();
    println!("addresses: {:?}", addresses);

    let address = addresses.keys.first().copied().unwrap();

    let result = quorum_vault_client::api::ethereum::read_account(&client, "quorum", address)
        .await
        .unwrap();
    println!("result: {:?}", result);

    let tx: TransactionRequest = TransactionRequest::default()
        .from(address)
        .to(address)
        .value(U256::from_str("1000000000000000000").unwrap())
        .gas_limit(21000)
        .gas_price(1)
        .nonce(0);

    let signature = quorum_vault_client::api::ethereum::sign_transaction(&client, "quorum", 1, tx)
        .await
        .unwrap();
    println!("signature: {:?}", signature);

    let data = b"Hello, world!";
    let signature2 = quorum_vault_client::api::ethereum::sign(&client, "quorum", address, data)
        .await
        .unwrap();

    println!("signature2: {:?}", signature2);
}
