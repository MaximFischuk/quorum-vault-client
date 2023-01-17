use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("Error: {0}")]
    VaultClientError(#[from] vaultrs::error::ClientError),
}
