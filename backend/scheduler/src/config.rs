#[derive(clap::Parser)]
pub struct Config {
    /// The connection URL for the Postgres database this application should use.
    #[clap(long, env)]
    pub database_url: String,

    /// MySQL max connections config
    #[clap(long, env)]
    pub max_connections: u32,
}
