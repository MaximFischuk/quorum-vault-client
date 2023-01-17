use vaultrs::client::{VaultClient, VaultClientSettingsBuilder};
use web3::types::{Address, TransactionRequest, U256};
use std::str::FromStr;
use std::io::Write;

#[tokio::main]
async fn main() {
    init_logger();
    let client = VaultClient::new(
        VaultClientSettingsBuilder::default()
            .address("http://127.0.0.1:8200")
            .token("s.roKCmV6b9cmSiUEM6wzdsRe8")
            .build()
            .unwrap()
    ).unwrap();

    let addresses = quorum_vault_client::api::list_accouns(&client, "quorum").await.unwrap();
    println!("addresses: {:?}", addresses);

    let address = Address::from_str("0x1daBe0aCaAA4D1F81b9b43Eaf51C8439378231a0").unwrap();

    let result = quorum_vault_client::api::read_account(&client, "quorum", "0x1daBe0aCaAA4D1F81b9b43Eaf51C8439378231a0").await.unwrap();
    println!("result: {:?}", result);

    let mut tx: TransactionRequest = TransactionRequest::builder()
        .from(address)
        .to(address)
        .value(U256::from_dec_str("1000000000000000000").unwrap())
        .gas(U256::from(21000))
        .nonce(U256::from(0))
        .build();

    tx.gas_price = Some(U256::from(1));

    let signature = quorum_vault_client::api::sign_transaction(&client, "quorum", tx).await.unwrap();
    println!("signature: {:?}", signature);
}

fn init_logger() {
    let mut builder = env_logger::Builder::from_default_env();
    builder.format(|buf, record| {
        writeln!(
            buf,
            "{} [{}] - {}",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
            record.level(),
            record.args()
        )
    });
    builder.filter_level(log::LevelFilter::Info);
    builder.init();
}
