use anchor_client::{
    solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey, signature::Keypair},
    Client, Cluster,
};
use sqlx::types::time::OffsetDateTime;
use std::rc::Rc;

pub const MAINNET_HTTP_URL: &str = "https://solana-api.projectserum.com";
pub const MAINNET_WS_URL: &str = "wss://solana-api.projectserum.com";
pub const DEVNET_HTTP_URL: &str = "https://api.devnet.solana.com";
pub const DEVNET_WS_URL: &str = "wss://api.devnet.solana.com";
pub const PROGRAM_ID: &str = "6dgkaNrtqjPbSzDTsDKErUV8JM45188MW756TVSDm2ZC";

pub fn build_client(network: String) -> Client {
    let cluster: Cluster;

    if network == "mainnet" {
        cluster = Cluster::Custom(MAINNET_HTTP_URL.to_string(), MAINNET_WS_URL.to_string());
    } else {
        cluster = Cluster::Custom(DEVNET_HTTP_URL.to_string(), DEVNET_WS_URL.to_string());
    }

    let commitment_config = CommitmentConfig::processed();
    let payer = Keypair::new();
    let client = Client::new_with_options(cluster, Rc::new(payer), commitment_config);

    return client;
}

pub struct ScheduleHelper {
    pub timestamp: u64, // Unix timestamp for when this specific schedule is active
    pub address: String,
    pub owner: String,
    pub from_token_mint: String,
    pub to_token_mint: String,
    pub owner_from_token_account: String,
    pub owner_to_token_account: String,
    pub vault_from_token_account: String,
    pub vault_to_token_account: String,
    pub amount_per_interval: u64,
    pub interval_length: u64,
    pub interval_counter: u16,
    pub max_intervals: u16,
    pub crank_authority: String,
    pub created_at: u64,
}
