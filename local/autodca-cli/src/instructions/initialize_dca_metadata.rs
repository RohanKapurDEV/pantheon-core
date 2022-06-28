use crate::utils::*;
use anchor_client::{
    anchor_lang::AccountDeserialize,
    solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer},
    Client, ClientError,
};
use anchor_spl::token::{Mint, TokenAccount};
use autodca::accounts as autodca_accounts;
use autodca::instruction as autodca_ixs;
use autodca::state::*;
use std::fs;
use std::str::FromStr;

pub async fn initialize_dca_metadata(
    client: &Client,
    keypair_path: String,
    amount_per_interval: u64,
    interval_length: u64,
    max_intervals: u16,
    crank_authority: String,
    from_mint: String,
    to_mint: String,
) -> Result<bool, ClientError> {
    // Validate inputs
    let from_mint = Pubkey::from_str(&from_mint).expect(INCORRECT_FORMAT_FOR_PUBKEY);
    let to_mint = Pubkey::from_str(&to_mint).expect(INCORRECT_FORMAT_FOR_PUBKEY);
    let crank_authority = Pubkey::from_str(&crank_authority).expect(INCORRECT_FORMAT_FOR_PUBKEY);

    let program_id_pubkey = Pubkey::from_str(PROGRAM_ID).unwrap();
    let system_program_pubkey = Pubkey::from_str(SYSTEM_PROGRAM).unwrap();
    let token_program_pubkey = Pubkey::from_str(TOKEN_PROGRAM).unwrap();
    let associated_token_program = Pubkey::from_str(ASSOCIATED_TOKEN_PROGRAM).unwrap();
    let rent = Pubkey::from_str(RENT).unwrap();

    let kp_data = fs::read_to_string(keypair_path).expect(CANNOT_READ_KEYPAIR);
    let kp_vector: Vec<u8> = serde_json::from_str(&kp_data).expect(INCORRECT_FORMAT_FOR_KEYPAIR);

    let payer_signer = Keypair::from_bytes(kp_vector.as_slice()).unwrap();
    let payer_pubkey = payer_signer.pubkey();

    let program = client.program(program_id_pubkey);

    // Check to see that from_mint is a valid mint account
    let from_mint_account = program.rpc().get_account(&from_mint)?;
    let data = from_mint_account.data.clone();
    let raw_bytes: &mut &[u8] = &mut &data[..];
    Mint::try_deserialize(raw_bytes).expect("from_mint is not a valid mint account. Try again");

    // Check to see that a token account for the for_mint exists and has sufficient balance to pay for the DCA
    let (from_mint_ata_pubkey, _from_mint_ata_bump) = Pubkey::find_program_address(
        &[
            &payer_pubkey.to_bytes(),
            &token_program_pubkey.to_bytes(),
            &from_mint.to_bytes(),
        ],
        &associated_token_program,
    );

    let from_mint_token_account = program.rpc().get_account(&from_mint_ata_pubkey)?;

    let data = from_mint_token_account.data.clone();
    let raw_bytes: &mut &[u8] = &mut &data[..];

    let token_account = TokenAccount::try_deserialize(raw_bytes).unwrap();

    let balance = token_account.amount;
    let dca_amount = amount_per_interval * max_intervals as u64;

    if balance < dca_amount {
        panic!(
            "Insufficient balance in token account for from_mint. Balance: {}, DCA amount: {}",
            balance, dca_amount
        );
    }

    let (program_as_signer, _program_as_signer_bump) =
        Pubkey::find_program_address(&[b"program", b"signer"], &program_id_pubkey);

    let dca_metadata_keypair = Keypair::new();
    let dca_metadata_pubkey = dca_metadata_keypair.pubkey();

    let (to_mint_ata_pubkey, _to_mint_ata_bump) = Pubkey::find_program_address(
        &[
            &payer_pubkey.to_bytes(),
            &token_program_pubkey.to_bytes(),
            &to_mint.to_bytes(),
        ],
        &associated_token_program,
    );

    let (from_mint_vault_token_account_pubkey, _from_mint_vault_token_account_pubkey_bump) =
        Pubkey::find_program_address(&[b"vault", &from_mint.to_bytes()], &program_id_pubkey);

    let (to_mint_vault_token_account_pubkey, _to_mint_vault_token_account_pubkey_bump) =
        Pubkey::find_program_address(&[b"vault", &to_mint.to_bytes()], &program_id_pubkey);

    // Run instruction
    let accounts = autodca_accounts::InitializeDcaMetadata {
        associated_token_program: associated_token_program,
        crank_authority: crank_authority,
        dca_metadata: dca_metadata_pubkey,
        from_mint: from_mint,
        to_mint: to_mint,
        from_mint_user_token_account: from_mint_ata_pubkey,
        from_mint_vault_token_account: from_mint_vault_token_account_pubkey,
        to_mint_vault_token_account: to_mint_vault_token_account_pubkey,
        to_mint_user_token_account: to_mint_ata_pubkey,
        payer: payer_pubkey,
        program_as_signer: program_as_signer,
        rent: rent,
        system_program: system_program_pubkey,
        token_program: token_program_pubkey,
    };

    let args = autodca_ixs::InitializeDcaMetadata {
        amount_per_interval: amount_per_interval,
        interval_length: interval_length,
        max_intervals: max_intervals,
    };

    let tx = program
        .request()
        .accounts(accounts)
        .args(args)
        .signer(&payer_signer)
        .signer(&dca_metadata_keypair)
        .send()?;

    // Print out the DCA metadata account

    let dca_metadata_account: DcaMetadata = program.account(dca_metadata_pubkey)?;

    println!("Tx sig: {}", tx.to_string());

    // Print out the DCA metadata account pubkey
    println!("DCA metadata pubkey: {}", dca_metadata_pubkey);

    // Print out the DCA metadata account interval_length
    println!(
        "DCA metadata interval_length: {}",
        dca_metadata_account.interval_length
    );

    // Print out DCA metadata account max_intervals
    println!(
        "DCA metadata max_intervals: {}",
        dca_metadata_account.max_intervals
    );

    // Print out DCA metadata account amount_per_interval
    println!(
        "DCA metadata amount_per_interval: {}",
        dca_metadata_account.amount_per_interval
    );

    // Print out DCA metadata account from_mint
    println!(
        "DCA metadata from_mint: {}",
        dca_metadata_account.from_token_mint
    );

    // Print out DCA metadata account to_mint
    println!(
        "DCA metadata to_mint: {}",
        dca_metadata_account.to_token_mint
    );

    Ok(true)
}
