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
    #[msg("Instruction not signed by current crank authority")]
    CurrentCrankAuthorityNotSigner,
    #[msg("The current crank authority does not own the token account")]
    CurrentCrankDoesNotOwnTokenAccount,
    #[msg("The current interval is higher than the max set by the user")]
    CurrentIntervalOutOfBounds,
    #[msg("The payment schedule initially set by the owner is being violated")]
    DcaScheduleInViolation,
    #[msg("Only the owner of the account can call the close instruction")]
    IncorrectOwner,
}
