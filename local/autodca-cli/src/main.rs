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
            EntityType::InitCrankAuthority(args) => {
                let network = args.network;
                let keypair_path = args.keypair_path;

                let client = build_client(keypair_path.clone(), network);
                let res = initialize_crank_authority(
                    &client,
                    keypair_path,
                    args.fee_bps,
                    args.crank_treasury,
                )
                .await;

                match res {
                    Ok(_) => {}
                    Err(e) => {
                        println!("{}", e.to_string())
                    }
                }
            }
            _ => {}
        },
    }
}
