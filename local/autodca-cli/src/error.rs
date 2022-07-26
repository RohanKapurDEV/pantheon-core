#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Network param must be mainnet or devnet")]
    NotValidNetwork,
    #[allow(dead_code)]
    #[error("Token account holds insufficient balance")]
    InsufficientBalance,
}
