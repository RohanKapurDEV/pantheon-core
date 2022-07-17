use anyhow::Context;
use clap::Parser;
use sqlx::mysql::{MySqlPoolOptions, MySqlQueryResult};
use std::time::{SystemTime, UNIX_EPOCH};

mod config;

use config::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let unix_timestamp = since_the_epoch.as_secs().to_string();
    // Parse our configuration from the environment.
    // This will exit with a help message if something is wrong.
    let config: Config = Config::parse();

    let db = MySqlPoolOptions::new()
        .max_connections(config.max_connections)
        .connect(&config.database_url)
        .await
        .context("could not connect to database_url")?;

    let try_fetching_schedules = sqlx::query!(
        "SELECT * FROM payment_schedule WHERE timestamp <= ?",
        unix_timestamp
    )
    .fetch_all(&db)
    .await;

    Ok(())
}

// NOTES:
// ok so still not sure how error handling will work. what to do when an api call fails? retry max like 5 times?

async fn fetch_schedules() -> Result<MySqlQueryResult, sqlx::Error> {
    todo!()
}
