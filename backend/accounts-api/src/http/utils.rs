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
pub const PROGRAM_ID: &str = "4AWRyt6whM4M8C4rimokJxvDP6bts7NxWNxZuyFVKD31";

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

// pub struct PaymentConfigSchema {
//     pub payment_config_id: u64,
//     pub address: String,
//     pub network: String,
//     pub inserted_at: OffsetDateTime,
//     pub updated_at: OffsetDateTime,
// }

// /// Helper struct containing all data that would be needed to call the collect_payments instruction for any
// /// given Config+Metadata pair
// pub struct CollectPaymentsHelper {
//     pub timestamp: i64,
//     pub merchant_authority_index: u8,
//     pub payment_config_index: u8,
//     pub merchant_authority_pubkey: Pubkey,
//     pub init_authority_pubkey: Pubkey,
//     pub payment_config_pubkey: Pubkey,
//     pub payment_metadata_pubkey: Pubkey,
//     pub payment_token_account_pubkey: Pubkey,
//     pub owner_payment_account_pubkey: Pubkey,
//     pub owner_pubkey: Pubkey,
// }
