use crate::http::utils::*;
use crate::http::ApiContext;
use crate::http::{Error, Result};
use anchor_client::anchor_lang::AccountDeserialize;
use anchor_client::solana_sdk::account::Account;
use anchor_client::solana_sdk::pubkey::Pubkey;
use autodca::state::DcaMetadata;
use axum::extract::{Extension, Query};
use axum::routing::{get, post};
use axum::{Json, Router};
use axum_macros::debug_handler;
use sqlx::mysql::MySqlQueryResult;
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

/// Router for all accounts endpoints
pub fn router() -> Router {
    Router::new()
        .route("/api/healthcheck", get(get_healthcheck))
        .route("/api/schedule", get(get_schedule_for_dca_metadata))
        .route("/api/accounts/dcaMetadata", post(post_dca_metadata))
}

#[derive(serde::Deserialize)]
struct NetworkParam {
    network: String,
}

#[derive(serde::Serialize)]
struct GetScheduleForDcaMetadataResponse {
    network: String,
    dca_metadata_address: String,
    schedule: Vec<u32>,
}

#[derive(serde::Deserialize)]
struct AddressAndNetworkParam {
    address: String,
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

/// Posts the dca metadata account and associated payment schedules to the database
///
/// Query param: network
/// Req body: DcaMetadataPostRequest<DcaMetadataPostBody>
#[debug_handler]
async fn post_dca_metadata(
    ctx: Extension<ApiContext>,
    Json(body): Json<DcaMetadataPostRequest>,
    Query(params): Query<NetworkParam>,
) -> Result<Json<bool>> {
    let address = body.dca_metadata.address.clone();
    let network_param = params.network.clone();

    if network_param != "mainnet" && network_param != "devnet" {
        return Err(Error::BadRequest);
    }

    let try_pubkey = Pubkey::from_str(&address);

    let mut db_schedules: Vec<ScheduleHelper> = Vec::new();
    let db_account: DcaMetadata;

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
                            "dca_metadata owner",
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
                // For now, return an error if the interval counter is not 0
                return Err(Error::unprocessable_entity([(
                    "interval counter",
                    "request.dca_metadata.address passed in was a valid account that is owned by the right program but is associated with a non-zero interval counter",
                )]));
            } else {
                let mut schedule_vector: Vec<ScheduleHelper> = Vec::new();

                for i in 0..dca_metadata_max_intervals {
                    let schedule_helper = ScheduleHelper {
                        // NOTE: Careful! This adds in the zero index as the first interval. Do not write that into
                        // the database
                        timestamp: dca_metadata_created_at
                            + (i as u64 * dca_metadata_interval_length),
                        address: pubkey.to_string(),
                        amount_per_interval: dca_metadata_amount_per_interval,
                        interval_counter: dca_metadata_interval_counter,
                        interval_length: dca_metadata_interval_length,
                        max_intervals: dca_metadata_max_intervals,
                        created_at: dca_metadata_created_at,
                        crank_authority: deserialized_dca_metadata.crank_authority.to_string(),
                        from_token_mint: deserialized_dca_metadata.from_token_mint.to_string(),
                        to_token_mint: deserialized_dca_metadata.to_token_mint.to_string(),
                        owner: deserialized_dca_metadata.owner.to_string(),
                        owner_from_token_account: deserialized_dca_metadata
                            .owner_from_token_account
                            .to_string(),
                        owner_to_token_account: deserialized_dca_metadata
                            .owner_to_token_account
                            .to_string(),
                        vault_from_token_account: deserialized_dca_metadata
                            .vault_from_token_account
                            .to_string(),
                        vault_to_token_account: deserialized_dca_metadata
                            .vault_to_token_account
                            .to_string(),
                    };

                    schedule_vector.push(schedule_helper);
                }

                // Add correct schedule items to db_schedules vector
                for item in schedule_vector {
                    if item.timestamp != dca_metadata_created_at {
                        db_schedules.push(item);
                    }
                }

                db_account = deserialized_dca_metadata;
            }
        }
        Err(_e) => {
            return Err(Error::BadRequest);
        }
    }

    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // Only try to schedule DCAs that are upcoming and not ones whose time has passed
    let db_schedules_sorted: Vec<ScheduleHelper> = db_schedules
        .iter()
        .filter(|x| x.timestamp >= current_time)
        .cloned()
        .collect();

    // Multiple instructions sent to database as a transaction
    // Sidenote: People cannot send duplicate schedules to the database. This is because the dca_metadata insert will fail and the transaction will rollback
    // so we don't need to handle that case in any direct manner
    let tx = ctx.db.begin().await?;

    let try_insert: Result<MySqlQueryResult, sqlx::Error> = sqlx::query!(
        r#"insert into dca_metadata (network, created_at, dca_metadata_address, owner_address, from_token_mint, to_token_mint, owner_from_token_account, owner_to_token_account, vault_from_token_account, vault_to_token_account, amount_per_interval, interval_length, interval_counter, max_intervals, crank_authority) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        params.network,
        db_account.created_at,
        address,
        db_account.owner.to_string(),
        db_account.from_token_mint.to_string(),
        db_account.to_token_mint.to_string(),
        db_account.owner_from_token_account.to_string(),
        db_account.owner_to_token_account.to_string(),
        db_account.vault_from_token_account.to_string(),
        db_account.vault_to_token_account.to_string(),
        db_account.amount_per_interval,
        db_account.interval_length,
        db_account.interval_counter,
        db_account.max_intervals,
        db_account.crank_authority.to_string(),
    )
    .execute(&ctx.db)
    .await;

    let id: u64;

    match try_insert {
        Ok(value) => {
            // Fetch id of the last insert into the database
            id = value.last_insert_id();
        }
        Err(e) => {
            println!("Error inserting into dca_metadata table: ${:?}", e);

            return Err(Error::unprocessable_entity([(
                "database error",
                "an error occured with the database, please try again",
            )]));
        }
    }

    for item in db_schedules_sorted {
        let try_insert: Result<MySqlQueryResult, sqlx::Error> = sqlx::query!(
            r#"insert into payment_schedule (network, timestamp, dca_metadata_id, dca_metadata_address) VALUES (?, ?, ?, ?)"#,
        params.network, item.timestamp, id, body.dca_metadata.address)
        .execute(&ctx.db)
        .await;

        match try_insert {
            Ok(_value) => {}
            Err(e) => {
                println!("Error inserting into payment_schedule table: ${:?}", e);

                // Rollback the entire transaction if a schedule insert fails
                tx.rollback().await?;

                return Err(Error::unprocessable_entity([(
                    "database error",
                    "an error occured with the database, please try again",
                )]));
            }
        }
    }

    // Commit multiple changes to the database
    tx.commit().await?;

    return Ok(Json(true));
}

/// Return all scheduled payments for a specific dca metadata account
///
/// Query param: network
/// Query param: dca_metadata_address
#[debug_handler]
async fn get_schedule_for_dca_metadata(
    ctx: Extension<ApiContext>,
    Query(params): Query<AddressAndNetworkParam>,
) -> Result<Json<GetScheduleForDcaMetadataResponse>> {
    let address_param = params.address;
    let network_param = params.network;

    // Validate network param
    if network_param != "mainnet" && network_param != "devnet" {
        return Err(Error::BadRequest);
    }

    // Check if address is a valid pubkey
    match Pubkey::from_str(&address_param) {
        Ok(value) => {
            let pubkey = value;

            let client = build_client(network_param.clone());
            let program = client.program(Pubkey::default());

            let account_res = program.rpc().get_account(&pubkey.clone());
            let raw_dca_metadata: Account;

            match account_res {
                Ok(account) => {
                    let owner = account.owner;

                    if owner.to_string() != PROGRAM_ID {
                        return Err(Error::unprocessable_entity([(
                    "dca_metadata owner",
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
                Ok(_account) => {}
                Err(e) => {
                    return Err(Error::unprocessable_entity([(
                        "deserializing dca_metadata",
                        e.to_string(),
                    )]));
                }
            }
        }

        Err(_e) => {
            return Err(Error::unprocessable_entity([(
                "address",
                "request.dca_metadata.address passed in was not a valid pubkey",
            )]));
        }
    };

    // What we want to do here is check the database for which sceduled payments are associated with this dca metadata account.
    let try_db_schedules_fetch = sqlx::query!(
        r#"select * from payment_schedule where dca_metadata_address = ?"#,
        address_param
    )
    .fetch_all(&ctx.db)
    .await;

    match try_db_schedules_fetch {
        Ok(db_schedules) => {
            let mut timestamp_vec: Vec<u32> = Vec::new();

            for item in db_schedules {
                timestamp_vec.push(item.timestamp);
            }

            return Ok(Json(GetScheduleForDcaMetadataResponse {
                network: network_param,
                dca_metadata_address: address_param,
                schedule: timestamp_vec,
            }));
        }
        Err(_e) => {
            return Err(Error::unprocessable_entity([(
                "database error",
                "an error occured with the database, please try again",
            )]));
        }
    }
}

/// Simple healthcheck endpoint for this microservice
async fn get_healthcheck() -> Result<Json<bool>> {
    Ok(Json(true))
}
