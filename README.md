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
    * Import Private Key
    * Sign Arbitrary Data
* Keys
    * Create Key
    * List Keys
    * Read Key
    * Delete Key
    * Sign Data
    * Import Private Key
    * Update Key Tags
* ZK-SNARKs
    * Create ZK-SNARKs Account
    * Read ZK-SNARKs Account
    * List ZK-SNARKs Accounts
    * Sign Data

## Migrating to v2.0

Version 2.0 is a major release that replaces the unmaintained [`web3`](https://crates.io/crates/web3) crate with the actively maintained [`alloy`](https://github.com/alloy-rs) ecosystem, updates the Rust edition to **2024**, and introduces stricter typing across the API.

### Breaking Changes at a Glance

| Area | v1.x | v2.0 |
|---|---|---|
| **Rust edition** | 2021 | 2024 |
| **Ethereum types** | `web3 = "0.19"` | `alloy-primitives = "1"` |
| **Transaction types** | `web3::types::TransactionRequest` | `alloy-rpc-types-eth::TransactionRequest` |
| **Address checksumming** | `eth_checksum` crate | Built-in `Address::to_checksum(None)` |
| **Keccak256 hashing** | `web3::signing::keccak256()` | `alloy_primitives::keccak256()` |
| **Re-exports** | `pub use web3::types::*` | `pub use alloy_primitives::*` |

### 1. Update your `Cargo.toml`

Remove `web3` from your dependencies and add `alloy-primitives` if you use Ethereum primitive types directly:

```toml
[dependencies]
quorum-vault-client = "2.0.0"
# Only needed if you construct alloy types directly:
# alloy-primitives = "1"
# alloy-rpc-types-eth = "1"
```

### 2. Update imports

The crate now re-exports types from `alloy_primitives` instead of `web3::types`:

```rust
// v1.x
use quorum_vault_client::{Client, VaultClient, VaultClientSettingsBuilder};
use web3::types::{Address, U256, TransactionRequest};

// v2.0
use quorum_vault_client::{Client, VaultClient, VaultClientSettingsBuilder, Address, U256, TransactionRequest};
```

All commonly used types (`Address`, `U256`, `Bytes`, etc.) keep the same names but come from `alloy_primitives`. If you were importing `web3::types::*` directly, replace those imports with `alloy_primitives::*`.

> **Note:** `H256` from web3 is replaced by `B256` in alloy.

### 3. Update `TransactionRequest` construction

The `TransactionRequest` builder API has changed significantly:

```rust
// v1.x
let mut tx = TransactionRequest::builder()
    .from(address)
    .to(address)
    .value(U256::from_dec_str("1000000000000000000").unwrap())
    .gas(U256::from(21000))
    .nonce(U256::from(0))
    .build();
tx.gas_price = Some(U256::from(1));

// v2.0
let tx = TransactionRequest::default()
    .from(address)
    .to(address)
    .value(U256::from_str("1000000000000000000").unwrap())
    .gas_limit(21000)
    .gas_price(1)
    .nonce(0);
```

Key differences:
- `TransactionRequest::builder().build()` → `TransactionRequest::default()` (no `.build()` call)
- `.gas(U256)` → `.gas_limit(u64)` — accepts a native integer
- `.nonce(U256)` → `.nonce(u64)` — accepts a native integer
- `.gas_price` is now set via the builder chain, not as a mutable field
- `U256::from_dec_str(...)` → `U256::from_str(...)` (requires `use std::str::FromStr`)

### 4. Update `import_private_key` calls

The `import_private_key` function now accepts `B256` instead of `&str` for type safety:

```rust
// v1.x
quorum_vault_client::api::ethereum::import_private_key(
    &client, "quorum", "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
).await.unwrap();

// v2.0
use quorum_vault_client::B256;

let private_key: B256 = "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
    .parse()
    .unwrap();
quorum_vault_client::api::ethereum::import_private_key(
    &client, "quorum", private_key
).await.unwrap();
```

### 5. API module paths

API functions are now organized under explicit submodules. Update all call sites:

```rust
// Ethereum
quorum_vault_client::api::ethereum::create_account(&client, "quorum").await?;
quorum_vault_client::api::ethereum::list_accounts(&client, "quorum").await?;
quorum_vault_client::api::ethereum::read_account(&client, "quorum", address).await?;
quorum_vault_client::api::ethereum::sign_transaction(&client, "quorum", chain_id, tx).await?;
quorum_vault_client::api::ethereum::import_private_key(&client, "quorum", private_key).await?;
quorum_vault_client::api::ethereum::sign(&client, "quorum", address, data).await?;

// Keys
quorum_vault_client::api::keys::create_key(&client, "quorum", id, algorithm, tags).await?;
quorum_vault_client::api::keys::read_key(&client, "quorum", id).await?;
quorum_vault_client::api::keys::list_keys(&client, "quorum").await?;
quorum_vault_client::api::keys::destroy_key(&client, "quorum", id).await?;
quorum_vault_client::api::keys::import_key(&client, "quorum", id, algorithm, tags, private_key).await?;
quorum_vault_client::api::keys::update_key_tags(&client, "quorum", id, tags).await?;
quorum_vault_client::api::keys::sign(&client, "quorum", id, data).await?;
quorum_vault_client::api::keys::sign_hash(&client, "quorum", id, hash).await?;

// ZK-SNARKs (new in v2.0)
quorum_vault_client::api::zksnarks::create_zksnarks_account(&client, "quorum").await?;
quorum_vault_client::api::zksnarks::read_zksnarks_account(&client, "quorum", id).await?;
quorum_vault_client::api::zksnarks::list_zksnarks_accounts(&client, "quorum").await?;
quorum_vault_client::api::zksnarks::zksnarks_sign(&client, "quorum", id, data).await?;
quorum_vault_client::api::zksnarks::zksnarks_sign_hash(&client, "quorum", id, hash).await?;
```

### 6. New features in v2.0

- **ZK-SNARKs support** — Full API for creating, reading, listing zk-SNARKs (EdDSA/BabyJubJub) accounts and signing data.
- **`sign` function for Ethereum** — Sign arbitrary data with an Ethereum account.
- **`sign_hash` functions** — Both Keys and ZK-SNARKs modules now provide `sign_hash` for signing pre-computed 32-byte digests, bypassing internal keccak256 hashing.
- **`update_key_tags`** — Update metadata tags on an existing key.

## Installation
Add the following to your `Cargo.toml`:
```toml
[dependencies]
quorum-vault-client = "2.0.0"
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
    let created_account = quorum_vault_client::api::ethereum::create_account(&client, "quorum").await.unwrap();
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

    let list_accounts = quorum_vault_client::api::ethereum::list_accounts(&client, "quorum").await.unwrap();
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
    let read_account = quorum_vault_client::api::ethereum::read_account(&client, "quorum", address).await.unwrap();
    println!("result: {:?}", read_account);
}
```

Result of the execution is the following:

```bash
> result: EthereumAccountResponse { address: 0x8d3113e29cb92f44f1762e52d2a0276509b36b82, compressed_public_key: "0x03b1c069a45b14697567661e6426ab0639f73762d7526765b2bd6891a73d84ebb5", public_key: "0x04b1c069a45b14697567661e6426ab0639f73762d7526765b2bd6891a73d84ebb57e6abbec4c9738a025d1a611e431ecf006227dbf6ca400f85518df70e5d101cb", namespace: "" }
```

**Sign Ethereum Transaction**

The following example signs an Ethereum Transaction.

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
    let tx = TransactionRequest::default()
        .from(address)
        .to(address)
        .value(U256::from_str("1000000000000000000").unwrap())
        .gas_limit(21000)
        .gas_price(1)
        .nonce(0);

    let sign_transaction = quorum_vault_client::api::ethereum::sign_transaction(&client, "quorum", 1, tx).await.unwrap();
    println!("result: {:?}", sign_transaction);
}
```

Result of the execution is the following:

```bash
> result: EthereumSignTransactionResponse { signature: "0xf29001752503d05ae83874193a8d866d49fc897c1a2fcb6229a0c61e4b5663f7097817a26f4c6014bbfd24c484bad9587c9c627c6f70d020f8638a4067bb78e801" }
```

**Import Private Key**

The following example imports an existing private key into the Vault as a new Ethereum account.

```rust
use quorum_vault_client::{Client, VaultClient, VaultClientSettingsBuilder, B256};

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

    let private_key: B256 = "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
        .parse()
        .unwrap();
    let account = quorum_vault_client::api::ethereum::import_private_key(&client, "quorum", private_key).await.unwrap();
    println!("result: {:?}", account);
}
```

Result of the execution is the following:

```bash
> result: EthereumAccountResponse { address: 0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266, compressed_public_key: "0x...", public_key: "0x...", namespace: "" }
```

**Sign Arbitrary Data**

The following example signs arbitrary data with an Ethereum account.

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
    let signature = quorum_vault_client::api::ethereum::sign(&client, "quorum", address, b"some-data").await.unwrap();
    println!("result: {:?}", signature);
}
```

Result of the execution is the following:

```bash
> result: EthereumSignResponse { signature: "0x..." }
```

### Keys

**Create Key**

The following example creates a new key in the Vault.

```rust
use quorum_vault_client::{Client, VaultClient, VaultClientSettingsBuilder};
use quorum_vault_client::api::keys::KeyCryptoAlgorithm;

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

    let created_key = quorum_vault_client::api::keys::create_key(
        &client,
        "quorum",
        "some-id",
        KeyCryptoAlgorithm::Secp256k1,
        [("tag".to_string(), "value".to_string())].into_iter().collect(),
    ).await.unwrap();
    println!("result: {:?}", created_key);
}
```

Result of the execution is the following:

```bash
> result: KeyResponse { created_at: "2023-01-30T09:08:22.217224856Z", curve: "secp256k1", id: "some-id", namespace: "", public_key: "BIwm5UiSGTiXVRlB_rS7qYSzQ6XZbaWfUOJKVicU85q-N7zuAak2JQfAHUs2Sm2WAA7YyWdN7_4UFJFggEa6AKw=", signing_algorithm: "ecdsa", tags: {"tag": "value"}, updated_at: "2023-01-30T09:08:22.217224856Z", version: 1 }
```

**Read Key**

The following example reads the key by id.

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

    let key = quorum_vault_client::api::keys::read_key(&client, "quorum", "some-id").await.unwrap();
    println!("result: {:?}", key);
}
```

Result of the execution is the following:

```bash
> result: KeyResponse { created_at: "2023-01-30T09:08:22.217224856Z", curve: "secp256k1", id: "some-id", namespace: "", public_key: "BIwm5UiSGTiXVRlB_rS7qYSzQ6XZbaWfUOJKVicU85q-N7zuAak2JQfAHUs2Sm2WAA7YyWdN7_4UFJFggEa6AKw=", signing_algorithm: "ecdsa", tags: {"tag": "value"}, updated_at: "2023-01-30T09:08:22.217224856Z", version: 1 }
```

**List Keys**

The following example lists all keys in the Vault.

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

    let keys = quorum_vault_client::api::keys::list_keys(&client, "quorum").await.unwrap();
    println!("result: {:?}", keys);
}
```

Result of the execution is the following:

```bash
> result: KeysResponse { keys: ["some-id"] }
```

**Delete Key**

The following example deletes the key by id.

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

    quorum_vault_client::api::keys::destroy_key(&client, "quorum", "some-id").await.unwrap();
}
```

**Sign Data**

The following example signs data with a key by its id. The data is hashed with keccak256 before signing.

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

    let signature = quorum_vault_client::api::keys::sign(&client, "quorum", "some-id", b"some-data").await.unwrap();
    println!("signature: {:?}", signature);
}
```

Result of the execution is the following:

```bash
> signature: SignResponse { signature: "Z1ibkBIGjMLh5pSR5mFZ5NbesrM57g-FGkFr0sbIyIlI_M0BYVN_LD-Nt7x1wUo6AoLQyL0I-z7PD8MsdgmkhQ==" }
```

**Import Private Key**

The following example imports an existing private key into the Vault.

```rust
use quorum_vault_client::{Client, VaultClient, VaultClientSettingsBuilder};
use quorum_vault_client::api::keys::KeyCryptoAlgorithm;

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

    let key = quorum_vault_client::api::keys::import_key(
        &client,
        "quorum",
        "some-id",
        KeyCryptoAlgorithm::Secp256k1,
        [("tag".to_string(), "value".to_string())].into_iter().collect(),
        "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80",
    ).await.unwrap();
    println!("result: {:?}", key);
}
```

Result of the execution is the following:

```bash
> result: KeyResponse { created_at: "2023-01-30T09:08:22.217224856Z", curve: "secp256k1", id: "some-id", namespace: "", public_key: "BIwm5UiSGTiXVRlB_rS7qYSzQ6XZbaWfUOJKVicU85q-N7zuAak2JQfAHUs2Sm2WAA7YyWdN7_4UFJFggEa6AKw=", signing_algorithm: "ecdsa", tags: {"tag": "value"}, updated_at: "2023-01-30T09:08:22.217224856Z", version: 1 }
```

**Update Key Tags**

The following example updates the metadata tags of an existing key.

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

    let key = quorum_vault_client::api::keys::update_key_tags(
        &client,
        "quorum",
        "some-id",
        [("env".to_string(), "production".to_string())].into_iter().collect(),
    ).await.unwrap();
    println!("result: {:?}", key);
}
```

Result of the execution is the following:

```bash
> result: KeyResponse { created_at: "2023-01-30T09:08:22.217224856Z", curve: "secp256k1", id: "some-id", namespace: "", public_key: "BIwm5UiSGTiXVRlB_rS7qYSzQ6XZbaWfUOJKVicU85q-N7zuAak2JQfAHUs2Sm2WAA7YyWdN7_4UFJFggEa6AKw=", signing_algorithm: "ecdsa", tags: {"env": "production"}, updated_at: "2023-01-30T09:08:22.217224856Z", version: 2 }
```

### ZK-SNARKs

**Create ZK-SNARKs Account**

The following example creates a new ZK-SNARKs (EdDSA/BabyJubJub) account in the Vault.

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

    let account = quorum_vault_client::api::zksnarks::create_zksnarks_account(&client, "quorum").await.unwrap();
    println!("result: {:?}", account);
}
```

Result of the execution is the following:

```bash
> result: ZkSnarksAccountResponse { curve: "babyjubjub", namespace: "", public_key: "...", signing_algorithm: "eddsa" }
```

**Read ZK-SNARKs Account**

The following example reads a ZK-SNARKs account by its public key, which is used as the account id.

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

    let account = quorum_vault_client::api::zksnarks::read_zksnarks_account(&client, "quorum", "<public-key>").await.unwrap();
    println!("result: {:?}", account);
}
```

Result of the execution is the following:

```bash
> result: ZkSnarksAccountResponse { curve: "babyjubjub", namespace: "", public_key: "...", signing_algorithm: "eddsa" }
```

**List ZK-SNARKs Accounts**

The following example lists all ZK-SNARKs accounts in the Vault.

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

    let accounts = quorum_vault_client::api::zksnarks::list_zksnarks_accounts(&client, "quorum").await.unwrap();
    println!("result: {:?}", accounts);
}
```

Result of the execution is the following:

```bash
> result: ZkSnarksAccountsResponse { keys: ["<public-key-1>", "<public-key-2>"] }
```

**Sign Data**

The following example signs data with a ZK-SNARKs account. The data is hashed with keccak256 before signing.

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

    let signature = quorum_vault_client::api::zksnarks::zksnarks_sign(&client, "quorum", "<public-key>", b"some-data").await.unwrap();
    println!("signature: {:?}", signature);
}
```

Result of the execution is the following:

```bash
> signature: ZkSnarksSignResponse { signature: "..." }
```
