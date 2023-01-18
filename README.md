# Quorum Vault Client

---

A Rust client for the [Quorum Vault Plugin](https://github.com/ConsenSys/quorum-hashicorp-vault-plugin) API.

> This client based on the [Vault Client](https://github.com/jmgilman/vaultrs).

The following backends are supported:
* Ethereum
    * Create Ethereum Account
    * List Ethereum Accounts
    * Read Ethereum Account by Address
    * Sign Ethereum Transaction (only Legacy)

## Installation
Add the following to your `Cargo.toml`:
```toml
[dependencies]
quorum-vault-client = "0.1.0"
```

## Usage

### Basic

The client is used to configure the connection to Vault and is required to
be passed to all API calls for execution. Behind the scenes it uses an
asynchronous client from [Reqwest](https://docs.rs/reqwest/) for
communicating to Vault.

```rust
use quorum_vault_client::{Client, VaultClient, VaultClientSettingsBuilder};

fn main() {
    let client = VaultClient::new(
        VaultClientSettingsBuilder::default()
            .address("https://127.0.0.1:8200")
            .token("TOKEN")
            .build()
            .unwrap()
    ).unwrap();
}
```

### Ethereum

**Create new Ethereum Wallet**

The following example creates a new Ethereum Wallet in the Vault.

```rust
use quorum_vault_client::{Client, VaultClient, VaultClientSettingsBuilder};

#[tokio::main]
async fn main() {
    // Create a client
    let client = VaultClient::new(
        VaultClientSettingsBuilder::default()
            .address("https://127.0.0.1:8200")
            .token("TOKEN")
            .build()
            .unwrap()
    ).unwrap();

    // By default the plugin mounts the Ethereum backend at the path "quorum"
    let created_account = quorum_vault_client::api::create_account(&client, "quorum").await.unwrap();
    println!("result: {:?}", created_account);
}

```

Result of the execution is the following:
```bash
> result: EthereumAccountResponse { address: 0x1a669bad7bda1f553087df8568b8782bcb0023ac, compressed_public_key: "0x020e44fde7435da96f8260788a89d4c37f2b3d96fd936dd978b886de6872d73062", public_key: "0x040e44fde7435da96f8260788a89d4c37f2b3d96fd936dd978b886de6872d730629c94a4803d3073b0bbe9a3d46f201eef5beec04d0e6f464e07704c159edd2c64", namespace: "" }
```

**List all Ethereum Wallets**

The following example gets list of all Ethereum Wallets in the Vault.

```rust
use quorum_vault_client::{Client, VaultClient, VaultClientSettingsBuilder};

#[tokio::main]
async fn main() {
    // Create a client
    let client = VaultClient::new(
        VaultClientSettingsBuilder::default()
            .address("https://127.0.0.1:8200")
            .token("TOKEN")
            .build()
            .unwrap()
    ).unwrap();

    let list_accounts = quorum_vault_client::api::list_accounts(&client, "quorum").await.unwrap();
    println!("result: {:?}", list_accounts);
}

```

Result of the execution is the following:

```bash
> result: EthereumAccountsResponse { keys: [0x1a669bad7bda1f553087df8568b8782bcb0023ac, 0x8d3113e29cb92f44f1762e52d2a0276509b36b82] }
```

**Read Ethereum Wallet**

The following example gets the Ethereum Wallet by address.

```rust
use quorum_vault_client::{Client, VaultClient, VaultClientSettingsBuilder, Address};
use std::str::FromStr;

#[tokio::main]
async fn main() {
    // Create a client
    let client = VaultClient::new(
        VaultClientSettingsBuilder::default()
            .address("https://127.0.0.1:8200")
            .token("TOKEN")
            .build()
            .unwrap()
    ).unwrap();

    let address = Address::from_str("0x8d3113e29CB92F44F1762E52D2a0276509b36b82").unwrap();
    let read_account = quorum_vault_client::api::read_account(&client, "quorum", account).await.unwrap();
    println!("result: {:?}", read_account);
}

```

Result of the execution is the following:

```bash
> result: EthereumAccountResponse { address: 0x8d3113e29cb92f44f1762e52d2a0276509b36b82, compressed_public_key: "0x03b1c069a45b14697567661e6426ab0639f73762d7526765b2bd6891a73d84ebb5", public_key: "0x04b1c069a45b14697567661e6426ab0639f73762d7526765b2bd6891a73d84ebb57e6abbec4c9738a025d1a611e431ecf006227dbf6ca400f85518df70e5d101cb", namespace: "" }
```

**Sign Ethereum Transaction**

The following example signs the Ethereum Transaction.

```rust
use quorum_vault_client::{Client, VaultClient, VaultClientSettingsBuilder, TransactionRequest, Address, U256};
use std::str::FromStr;

#[tokio::main]
async fn main() {
    // Create a client
    let client = VaultClient::new(
        VaultClientSettingsBuilder::default()
            .address("https://127.0.0.1:8200")
            .token("TOKEN")
            .build()
            .unwrap()
    ).unwrap();

    let address = Address::from_str("0x8d3113e29CB92F44F1762E52D2a0276509b36b82").unwrap();
    let mut tx: TransactionRequest = TransactionRequest::builder()
        .from(address)
        .to(address)
        .value(U256::from_dec_str("1000000000000000000").unwrap())
        .gas(U256::from(21000))
        .nonce(U256::from(0))
        .build();

    tx.gas_price = Some(U256::from(1));

    let sign_transaction = quorum_vault_client::api::sign_transaction(&client, "quorum", 1, &tx).await.unwrap();
    println!("result: {:?}", sign_transaction);
}

```

Result of the execution is the following:

```bash
> signature: EthereumSignTransactionResponse { signature: "0xf29001752503d05ae83874193a8d866d49fc897c1a2fcb6229a0c61e4b5663f7097817a26f4c6014bbfd24c484bad9587c9c627c6f70d020f8638a4067bb78e801" }
```