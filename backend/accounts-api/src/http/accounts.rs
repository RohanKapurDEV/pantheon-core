use crate::http::error::ResultExt;
use crate::http::utils::*;
use crate::http::ApiContext;
use crate::http::{Error, Result};
use anchor_client::solana_sdk::account::Account;
use anchor_client::solana_sdk::program_error::ProgramError;
use anchor_client::solana_sdk::program_pack::Pack;
use anchor_client::solana_sdk::pubkey::Pubkey;
use anchor_client::ClientError;
use anchor_client::{anchor_lang::AccountDeserialize, Client, Program};
use autodca::state::{CrankAuthority, DcaMetadata};
use axum::extract::{Extension, Path, Query};
use axum::routing::{get, post};
use axum::{Json, Router};
use axum_macros::debug_handler;
use sqlx::mysql::MySqlQueryResult;
use std::str::FromStr;

/// Router for all accounts endpoints
pub fn router() -> Router {
    Router::new()
        .route("/api/bb8", post(get_healthcheck))
        .route("/api/accounts/dcaMetadata", post(post_dca_metadata))
}

#[derive(serde::Deserialize)]
struct NetworkParam {
    network: String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct DcaMetadataPostRequest {
    dca_metadata: DcaMetadataPostBody,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DcaMetadataPostBody {
    pub address: String,
}

/// Posts the payment config account to the database
#[debug_handler]
async fn post_dca_metadata(
    ctx: Extension<ApiContext>,
    Json(body): Json<DcaMetadataPostRequest>,
    Query(params): Query<NetworkParam>,
) -> Result<Json<bool>> {
    let address = body.dca_metadata.address;
    let network_param = params.network.clone();

    if network_param != "mainnet" && network_param != "devnet" {
        return Err(Error::BadRequest);
    }

    let try_pubkey = Pubkey::from_str(&address);

    match try_pubkey {
        Ok(pubkey) => {
            let client = build_client(network_param);
            let program = client.program(Pubkey::default());

            // Validate pubkey does belong to correct program onchain
            let account_res = program.rpc().get_account(&pubkey);
            let raw_dca_metadata: Account;
            let deserialized_dca_metadata: DcaMetadata;

            match account_res {
                Ok(account) => {
                    let owner = account.owner;

                    if owner.to_string() != PROGRAM_ID {
                        return Err(Error::unprocessable_entity([(
                            "payment_config owner",
                            "request.dca_metadata.address passed in was a valid account but is owned by the wrong program",
                        )]));
                    }

                    raw_dca_metadata = account;
                }
                Err(e) => {
                    return Err(Error::unprocessable_entity([(
                        "solana client error",
                        e.to_string(),
                    )]));
                }
            }

            let raw_bytes: &mut &[u8] = &mut &raw_dca_metadata.data[..];
            let try_deserialize_dca_metadata: Result<
                DcaMetadata,
                anchor_client::anchor_lang::error::Error,
            > = autodca::state::DcaMetadata::try_deserialize(raw_bytes);

            // Validate that the pubkey is an account of type `DcaMetadata`
            match try_deserialize_dca_metadata {
                Ok(account) => {
                    deserialized_dca_metadata = account;
                }
                Err(e) => {
                    return Err(Error::unprocessable_entity([(
                        "deserializing payment config",
                        e.to_string(),
                    )]));
                }
            }

            // Validate that the pubkey is associated with correct CrankAuthority from ApiContext
            let associated_crank_authority = deserialized_dca_metadata.crank_authority;

            if associated_crank_authority.to_string() != ctx.config.crank_authority.clone() {
                return Err(Error::unprocessable_entity([(
                    "associated crank authority",
                    "request.dca_metadata.address passed in was a valid account that is owned by the right program but is associated with the wrong crank authority",
                )]));
            }

            let dca_metadata_created_at = deserialized_dca_metadata.created_at;
            let dca_metadata_amount_per_interval = deserialized_dca_metadata.amount_per_interval;
            let dca_metadata_interval_length = deserialized_dca_metadata.interval_length;
            let dca_metadata_interval_counter = deserialized_dca_metadata.interval_counter;
            let dca_metadata_max_intervals = deserialized_dca_metadata.max_intervals;

            if dca_metadata_interval_counter != 0 {
                // Handle case where certain schedules have already been executed
            } else {
            }
        }
        Err(_e) => {
            return Err(Error::BadRequest);
        }
    }

    // Insert into c3po
    // let try_insert: Result<MySqlQueryResult, sqlx::Error> = sqlx::query!(
    //     r#"insert into payment_config (address, network) VALUES (?, ?)"#,
    //     address,
    //     params.network
    // )
    // .execute(&ctx.db)
    // .await;

    // match try_insert {
    //     Ok(_value) => {}
    //     Err(_e) => {
    //         return Err(Error::unprocessable_entity([(
    //             "database error",
    //             "an error occured with the database, please try again",
    //         )]));
    //     }
    // }

    // return Ok(Json(true));

    todo!();
}

/// Posts the payment metadata account to the database and populates the payment_collections table appropriately
#[debug_handler]
async fn post_payment_metadata(
    ctx: Extension<ApiContext>,
    Json(body): Json<bool>,
    Query(params): Query<NetworkParam>,
) -> Result<Json<bool>> {
    todo!()
    // let address = body.payment_metadata.address;
    // let merchant_authority = &ctx.config.merchant_authority;
    // let network_param = params.network.clone();
    // let program_pubkey = Pubkey::from_str(PROGRAM_ID).unwrap();

    // let associated_payment_config_address: Pubkey;

    // let mut schedule_vector: Vec<CollectPaymentsHelper> = Vec::new();

    // if network_param != "mainnet" && network_param != "devnet" {
    //     return Err(Error::BadRequest);
    // }

    // let try_pubkey = Pubkey::from_str(&address);

    // match try_pubkey {
    //     Ok(pubkey) => {
    //         let client = build_client(network_param);
    //         let program = client.program(program_pubkey);

    //         // Validate pubkey does belong to correct program onchain
    //         let account_res = program.rpc().get_account(&pubkey);
    //         let raw_payment_metadata: Account;
    //         let deserialized_payment_metadata: PaymentMetadata;

    //         match account_res {
    //             Ok(account) => {
    //                 let owner = account.owner;

    //                 if owner.to_string() != PROGRAM_ID {
    //                     return Err(Error::unprocessable_entity([(
    //                         "payment_metadata owner",
    //                         "request.payment_metadata.address passed in was a valid account but is owned by the wrong program",
    //                     )]));
    //                 }

    //                 raw_payment_metadata = account;
    //             }
    //             Err(e) => {
    //                 return Err(Error::unprocessable_entity([(
    //                     "solana client",
    //                     e.to_string(),
    //                 )]));
    //             }
    //         }

    //         let raw_bytes: &mut &[u8] = &mut &raw_payment_metadata.data[..];
    //         let try_deserialize_payment_metadata: Result<
    //             PaymentMetadata,
    //             anchor_client::anchor_lang::error::Error,
    //         > = recurring::state::PaymentMetadata::try_deserialize(raw_bytes);

    //         // Validate that the pubkey is an account of type `PaymentMetadata`
    //         match try_deserialize_payment_metadata {
    //             Ok(account) => {
    //                 deserialized_payment_metadata = account;
    //             }
    //             Err(e) => {
    //                 return Err(Error::unprocessable_entity([(
    //                     "deserializing payment metadata",
    //                     e.to_string(),
    //                 )]));
    //             }
    //         }

    //         let associated_payment_config: PaymentConfig;
    //         associated_payment_config_address = deserialized_payment_metadata.payment_config;

    //         let associated_payment_config_res: Result<PaymentConfig, anchor_client::ClientError> =
    //             program.account(associated_payment_config_address);

    //         match associated_payment_config_res {
    //             Ok(payment_config) => associated_payment_config = payment_config,
    //             Err(_e) => {
    //                 // Handle error case for the payment_config now being deleted or non-existent
    //                 return Err(Error::unprocessable_entity([(
    //                     "payment_config",
    //                     "the associated payment_config for request.payment_metadata.address could not be fetched",
    //                 )]));
    //             }
    //         }

    //         // Look at schedule encoded in PaymentConfig account and build CollectPaymentsHelper vector
    //         let spacing_period = associated_payment_config.spacing_period;
    //         let created_at = deserialized_payment_metadata.created_at;
    //         let amount_delegated = deserialized_payment_metadata.amount_delegated;
    //         let amount_to_collect_per_period =
    //             associated_payment_config.amount_to_collect_per_period;

    //         let mint_decimals: u8;
    //         let token_mint = associated_payment_config.payment_mint;

    //         let mint_account = program.rpc().get_account(&token_mint);

    //         match mint_account {
    //             Ok(account) => {
    //                 let data = &account.data[..];
    //                 let deserialize_mint_res = spl_token::state::Mint::unpack(data);

    //                 match deserialize_mint_res {
    //                     Ok(mint) => mint_decimals = mint.decimals,
    //                     Err(_e) => {
    //                         return Err(Error::unprocessable_entity([(
    //                             "mint account",
    //                             "unable to deserialize the fetched mint account associated with the payment_config account associated with the payment_metadata account",
    //                         )]));
    //                     }
    //                 }
    //             }
    //             Err(_e) => {
    //                 return Err(Error::unprocessable_entity([(
    //                     "mint account",
    //                     "unable to fetch the mint account associated with the payment_config account associated with the payment_metadata account",
    //                 )]));
    //             }
    //         }

    //         let associated_merchant_authority: MerchantAuthority;
    //         let associated_merchant_authority_pubkey = associated_payment_config.merchant_authority;

    //         let associated_merchant_authority_res: Result<MerchantAuthority, ClientError> =
    //             program.account(associated_merchant_authority_pubkey);

    //         match associated_merchant_authority_res {
    //             Ok(merchant_authority) => associated_merchant_authority = merchant_authority,
    //             Err(_e) => {
    //                 return Err(Error::unprocessable_entity([(
    //                     "merchant_authority",
    //                     "the associated merchant_authority for request.payment_metadata.address could not be fetched",
    //                 )]));
    //             }
    //         }

    //         let ui_amount_to_collect_per_period =
    //             spl_token::amount_to_ui_amount(amount_to_collect_per_period, mint_decimals);

    //         let ui_amount_delegated =
    //             spl_token::amount_to_ui_amount(amount_delegated, mint_decimals);

    //         let amount_of_payments_to_schedule =
    //             (ui_amount_delegated / ui_amount_to_collect_per_period).floor() as u64;

    //         for index in 0..amount_of_payments_to_schedule {
    //             let applied_payments_index = index + 1;
    //             let timestamp = created_at + (applied_payments_index as i64 * spacing_period);

    //             schedule_vector.push(CollectPaymentsHelper {
    //                 timestamp,
    //                 payment_config_index: associated_payment_config.index,
    //                 payment_config_pubkey: associated_payment_config_address,
    //                 owner_pubkey: deserialized_payment_metadata.owner,
    //                 owner_payment_account_pubkey: deserialized_payment_metadata
    //                     .owner_payment_account,
    //                 payment_token_account_pubkey: associated_payment_config.payment_token_account,
    //                 payment_metadata_pubkey: pubkey,
    //                 merchant_authority_index: associated_merchant_authority.index,
    //                 merchant_authority_pubkey: associated_merchant_authority_pubkey,
    //                 init_authority_pubkey: associated_merchant_authority.init_authority,
    //             })
    //         }
    //     }
    //     Err(_e) => {
    //         return Err(Error::BadRequest);
    //     }
    // }

    // Write account into payment_metadata table in the database
    // let try_config: Result<PaymentConfigSchema, sqlx::Error> = sqlx::query_as!(
    //     PaymentConfigSchema,
    //     r#"select * from payment_config where address = ?"#,
    //     associated_payment_config_address.to_string()
    // )
    // .fetch_one(&ctx.db)
    // .await;

    // let associated_payment_config_id: u64;

    // match try_config {
    //     Ok(config_object) => {
    //         associated_payment_config_id = config_object.payment_config_id;
    //     }
    //     Err(_e) => {
    //         return Err(Error::unprocessable_entity([(
    //             "database error",
    //             "an error occured with the database, please try again",
    //         )]));
    //     }
    // }

    // let try_insert: Result<MySqlQueryResult, sqlx::Error> = sqlx::query!(
    //     r#"insert into payment_metadata (address, network, payment_config_id) VALUES (?, ?, ?)"#,
    //     address,
    //     params.network,
    //     associated_payment_config_id
    // )
    // .execute(&ctx.db)
    // .await;

    // match try_insert {
    //     Ok(_query_res) => {}
    //     Err(_e) => {
    //         return Err(Error::unprocessable_entity([(
    //             "database error",
    //             "an error occured with the database, please try again",
    //         )]));
    //     }
    // }

    // // Write schedule into payments_schedule table in the database using db transaction

    // return Ok(Json(true));
}

/// Return all scheduled payments for a specific payment metadata account
async fn get_schedule_for_payment_metadata() -> Result<Json<bool>> {
    todo!()
}

/// Simple healthcheck endpoint for this microservice
async fn get_healthcheck() -> Result<Json<bool>> {
    Ok(Json(true))
}
