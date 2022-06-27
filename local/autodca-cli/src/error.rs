#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Network param must be mainnet or devnet")]
    NotValidNetwork,
    #[error("Token account holds insufficient balance")]
    InsufficientBalance,
}
