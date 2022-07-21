use anyhow::Context;
use clap::Parser;
use sqlx::mysql::MySqlPoolOptions;
use std::time::{SystemTime, UNIX_EPOCH};

mod config;

use config::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let unix_timestamp = since_the_epoch.as_secs();
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
    .await?;

    for i in try_fetching_schedules {
        // Run the payment schedule through the TS api
        println!("{:?}", i);
    }

    Ok(())
}
