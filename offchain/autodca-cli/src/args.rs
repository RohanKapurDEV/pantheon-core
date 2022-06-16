use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(
    author = "Rohan Kapur",
    version,
    about = "A CLI application to interact with the autodca program\nTwitter: @0xrohan"
)]
pub struct ClientArgs {
    /// Initialize Merchant Account
    #[clap(subcommand)]
    pub subcommand: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// Initialize a CrankAuthority account
    InitCrankAuthority(CrankAuthorityParams),
    /// Initialize a DcaMetadata account
    InitDcaMetadata(InitDcaMetadataParams),
}

#[derive(Debug, Args)]
pub struct CrankAuthorityParams {
    /// The path to the JSON keypair to use the sign the transaction
    #[clap(long)]
    pub keypair_path: String,
    /// Fee bps for the CrankAuthority account
    #[clap(long)]
    pub fee_bps: u16,
    // Current authority for the CrankAuthority account
    #[clap(long)]
    pub current_authority: String,
    // Crank treasury account for the CrankAuthority account
    #[clap(long)]
    pub crank_treasury: String,
    /// mainnet or devnet
    #[clap(long)]
    pub network: String,
}

#[derive(Debug, Args)]
pub struct InitDcaMetadataParams {
    /// The path to the JSON keypair to use the sign the transaction
    #[clap(long)]
    pub keypair_path: String,
    /// Token size - take decimals into account!!!
    #[clap(long)]
    pub amount_per_interval: u64,
    /// Time in seconds between each purchase
    #[clap(long)]
    pub interval_length: u64,
    /// Amount of intervals for the DCA strategy
    #[clap(long)]
    pub max_intervals: u16,
    /// CrankAuthority account address to associate the DcaMetadata account with
    #[clap(long)]
    pub crank_authority: String,
    /// From mint
    #[clap(long)]
    pub from_mint: String,
    /// To mint
    #[clap(long)]
    pub to_mint: String,
    /// mainnet or devnet
    #[clap(long)]
    pub network: String,
}
