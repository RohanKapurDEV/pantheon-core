use anchor_lang::prelude::*;

#[account]
pub struct CrankAuthority {
    pub current_authority: Pubkey,
    pub pending_authority: Pubkey,
    pub crank_treasury: Pubkey,
    pub crank_fee_bps: u16,
}

impl CrankAuthority {
    pub const SIZE: usize = 8 + (32 * 3);
}

/// Total amount delegated = amount_per_interval * max_intervals
///
/// The entire payment schedule and it's particulars can be derived from this struct. The instruction
/// where the crank authority is able to temporally extract funds from this account's associated token
/// accounts should enforce that the schedule is valid and that the crank cannot withdraw more than
/// the account dictates.
#[account]
pub struct DcaMetadata {
    pub owner: Pubkey,
    pub from_token_mint: Pubkey,
    pub to_token_mint: Pubkey,
    pub owner_from_token_account: Pubkey,
    pub owner_to_token_account: Pubkey,
    pub contract_from_token_account: Pubkey,
    pub contract_to_token_account: Pubkey,
    pub amount_per_interval: u64,
    pub interval_length: u64,  // In seconds, duration between intervals
    pub interval_counter: u16, // Current interval index
    pub max_intervals: u16,    // Total amount of intervals
    pub crank_authority: Pubkey,
    pub created_at: i64,
}

impl DcaMetadata {
    pub const SIZE: usize = (32 * 8) + (8 * 3) + (2 * 2);
}
