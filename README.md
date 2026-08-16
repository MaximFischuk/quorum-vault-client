# Quorum Vault Client

Rust client for Vault signer plugin.

## API

All calls target Vault mount `signer`:

- `create_key`, `list_keys`, `read_key`, `delete_key`
- `sign_hash`, `sign_batch`, `sign_message`
- `sign_ethereum_transaction`, `sign_typed_data`, `sign_user_operation`

```rust
use std::collections::HashMap;

use quorum_vault_client::api::{self, KeyCurve};

let key = api::create_key(
    &client,
    "secp256k1",
    KeyCurve::Secp256k1,
    HashMap::from([(String::from("owner"), String::from("Alice"))]),
)
.await?;
```

See `examples/signer.rs` for client setup.
