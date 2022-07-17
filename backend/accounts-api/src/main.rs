// We prefer to keep `main.rs` and `lib.rs` separate as it makes it easier to add extra helper
// binaries later which share code with the main project. It could save you from a nontrivial
// refactoring effort in the future.
//
// Whether to make `main.rs` just a thin shim that awaits a `run()` function in `lib.rs`, or
// to put the application bootstrap logic here is an open question. Both approaches have their
// upsides and their downsides. Your input is welcome!

use std::str::FromStr;

use anchor_client::solana_sdk::pubkey::Pubkey;
use anyhow::Context;
use clap::Parser;
use sqlx::mysql::MySqlPoolOptions;

use accounts_api::config::Config;
use accounts_api::http;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // This returns an error if the `.env` file doesn't exist, but that's not what we want
    // since we're not going to use a `.env` file if we deploy this application.
    dotenv::dotenv().ok();

    // Initialize the logger.
    env_logger::init();

    // Parse our configuration from the environment.
    // This will exit with a help message if something is wrong.
    let config = Config::parse();

    Pubkey::from_str(&config.crank_authority)
        .expect("Invalid crank authority pubkey supplied to program");

    // We create a single connection pool for SQLx that's shared across the whole application.
    // This saves us from opening a new connection for every API call, which is wasteful.
    let db = MySqlPoolOptions::new()
        .max_connections(config.max_connections)
        .connect(&config.database_url)
        .await
        .context("could not connect to database_url")?;

    // This embeds database migrations in the application binary so we can ensure the database
    // is migrated correctly on startup. Currently disabled because it behaves awkwardly with
    // planetscale for some reason, but I'll fix that later.
    // sqlx::migrate!().run(&db).await?;

    // Finally, we spin up our API.
    http::serve(config, db).await?;

    Ok(())
}
