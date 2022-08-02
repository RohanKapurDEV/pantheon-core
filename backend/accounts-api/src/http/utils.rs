use anchor_client::{
    solana_sdk::{commitment_config::CommitmentConfig, signature::Keypair},
    Client, Cluster,
};
use std::{rc::Rc, sync::Arc};

use crate::config::Config;

pub const PROGRAM_ID: &str = "dca6xdPrxUTazoTEq7ue51nhWWSH2efXRBJhYrxHB4W";

pub fn build_client(network: String, config: Arc<Config>) -> Client {
    let cluster: Cluster;

    if network == "mainnet" {
        cluster = Cluster::Custom(
            config.mainnet_http_url.clone(),
            config.mainnet_ws_url.clone(),
        );
    } else {
        cluster = Cluster::Custom(config.devnet_http_url.clone(), config.devnet_ws_url.clone());
    }

    let commitment_config = CommitmentConfig::processed();
    let payer = Keypair::new();
    let client = Client::new_with_options(cluster, Rc::new(payer), commitment_config);

    return client;
}

#[derive(Debug, Clone)]
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
