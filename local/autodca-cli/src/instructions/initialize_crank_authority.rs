use crate::utils::*;
use anchor_client::{
    solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer},
    Client, ClientError,
};
use autodca::accounts as autodca_accounts;
use autodca::instruction as autodca_ixs;
use autodca::state::*;
use std::fs;
use std::str::FromStr;

pub async fn initialize_crank_authority(
    client: &Client,
    keypair_path: String,
    fee_bps: u16,
    crank_treasury: String,
) -> Result<bool, ClientError> {
    let program_id_pubkey = Pubkey::from_str(PROGRAM_ID).unwrap();
    let system_program_pubkey = Pubkey::from_str(SYSTEM_PROGRAM).unwrap();

    let kp_data = fs::read_to_string(keypair_path).expect(CANNOT_READ_KEYPAIR);
    let kp_vector: Vec<u8> = serde_json::from_str(&kp_data).expect(INCORRECT_FORMAT_FOR_KEYPAIR);

    let payer_signer = Keypair::from_bytes(kp_vector.as_slice()).unwrap();
    let payer_pubkey = payer_signer.pubkey();

    let program = client.program(program_id_pubkey);

    let crank_authority_keypair = Keypair::new();
    let crank_authority_pubkey = crank_authority_keypair.pubkey();

    let crank_treasury_keypair =
        Pubkey::from_str(&crank_treasury).expect(INCORRECT_FORMAT_FOR_PUBKEY);

    let accounts = autodca_accounts::InitializeCrankAuthority {
        crank_authority: crank_authority_pubkey,
        crank_treasury: crank_treasury_keypair,
        current_authority: payer_pubkey,
        payer: payer_pubkey,
        system_program: system_program_pubkey,
    };

    let params = autodca_ixs::InitializeCrankAuthority { fee_bps };

    let tx = program
        .request()
        .accounts(accounts)
        .args(params)
        .signer(&payer_signer)
        .signer(&crank_authority_keypair)
        .send()?;

    let crank_authority_account: CrankAuthority = program.account(crank_authority_pubkey)?;

    println!("Tx sig: {}", tx.to_string());

    println!("Crank authority pubkey: {}", crank_authority_pubkey);
    println!(
        "Current authority: {}",
        crank_authority_account.current_authority
    );
    println!(
        "Crank treasury: {}",
        crank_authority_account.current_authority
    );
    println!(
        "Fee basis points: {}",
        crank_authority_account.crank_fee_bps
    );
    println!(
        "Pending authority: {}",
        crank_authority_account.pending_authority
    );

    Ok(true)
}
