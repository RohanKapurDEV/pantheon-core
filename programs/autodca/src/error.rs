use anchor_lang::prelude::*;

#[error_code]
pub enum AutoDcaError {
    #[msg("Fee bps must be below 10000")]
    InvalidFeeBpsParameter,
    #[msg("Invalid crank authority")]
    InvalidCrankAuthority,
    #[msg("Invalid pending authority")]
    InvalidPendingAuthority,
    #[msg("From and To token mints cannot be the same")]
    TokenMintsCannotBeTheSame,
    #[msg("Incorrect token mint supplied")]
    IncorrectMint,
}
