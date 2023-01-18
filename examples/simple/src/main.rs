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
            .token("s.abx7eCduabnQNgMJZs5TECdi")
            .build()
            .unwrap()
    ).unwrap();

    let created_account = quorum_vault_client::api::create_account(&client, "quorum").await.unwrap();
    println!("result: {:?}", created_account);

    let addresses = quorum_vault_client::api::list_accouns(&client, "quorum").await.unwrap();
    println!("addresses: {:?}", addresses);

    let address = Address::from_str("0x8d3113e29CB92F44F1762E52D2a0276509b36b82").unwrap();

    let result = quorum_vault_client::api::read_account(&client, "quorum", address).await.unwrap();
    println!("result: {:?}", result);

    let mut tx: TransactionRequest = TransactionRequest::builder()
        .from(address)
        .to(address)
        .value(U256::from_dec_str("1000000000000000000").unwrap())
        .gas(U256::from(21000))
        .nonce(U256::from(0))
        .build();

    tx.gas_price = Some(U256::from(1));

    let signature = quorum_vault_client::api::sign_transaction(&client, "quorum", 1, tx).await.unwrap();
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
