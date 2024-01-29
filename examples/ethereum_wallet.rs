use vaultrs::client::{VaultClient, VaultClientSettingsBuilder};
use web3::types::{TransactionRequest, U256};

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

    let mut tx: TransactionRequest = TransactionRequest::builder()
        .from(address)
        .to(address)
        .value(U256::from_dec_str("1000000000000000000").unwrap())
        .gas(U256::from(21000))
        .nonce(U256::from(0))
        .build();

    tx.gas_price = Some(U256::from(1));

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
