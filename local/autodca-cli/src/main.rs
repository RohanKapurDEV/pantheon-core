mod args;
mod error;
mod instructions;
mod utils;

use args::*;
use error::*;
use instructions::*;
use utils::*;

use clap::Parser;

#[tokio::main]
async fn main() {
    let args = ClientArgs::parse();

    match args {
        ClientArgs { subcommand } => match subcommand {
            EntityType::InitCrankAuthority(args) => {}
            _ => {}
        },
    }
}
