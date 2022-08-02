#[derive(clap::Parser)]
pub struct Config {
    /// The connection URL for the Postgres database this application should use.
    #[clap(long, env)]
    pub database_url: String,

    /// The crank_authority account address we are validating incoming requests with (where applicable).
    ///
    /// See endpoint logic for where this is used
    #[clap(long, env)]
    pub crank_authority: String,

    /// MySQL max connections config
    #[clap(long, env)]
    pub max_connections: u32,

    #[clap(long, env)]
    pub mainnet_http_url: String,

    #[clap(long, env)]
    pub mainnet_ws_url: String,

    #[clap(long, env)]
    pub devnet_http_url: String,

    #[clap(long, env)]
    pub devnet_ws_url: String,
}
