use log::info;
use actix_web::{web, HttpResponse, Responder};
use alloy::primitives::{Address, U256, Bytes};
use chrono::Utc;
use fhe::bfv::BfvParametersBuilder;
use fhe_traits::Serialize;
use crate::enclave_server::blockchain::relayer::EnclaveContract;
use crate::enclave_server::config::CONFIG;
use crate::enclave_server::models::{CTRequest, RoundCount, PKRequest, CronRequestE3, JsonResponse};
use crate::enclave_server::database::{get_e3, get_e3_round};

pub fn setup_routes(config: &mut web::ServiceConfig) {
    config
        .route("/get_rounds", web::get().to(get_rounds))
        .route("/get_pk_by_round", web::post().to(get_pk_by_round))
        .route("/get_ct_by_round", web::post().to(get_ct_by_round))
        .route("/request_e3_round", web::post().to(request_e3_round));
}

async fn request_e3_round(
    data: web::Json<CronRequestE3>
) -> impl Responder {
    // Check API key
    if data.cron_api_key != CONFIG.cron_api_key {
        return HttpResponse::Unauthorized().json(JsonResponse {
            response: "Invalid API key".to_string(),
        });
    }

    // Initialize a new E3 round
    match initialize_crisp_round().await {
        Ok(_) => HttpResponse::Ok().json(JsonResponse {
            response: "New E3 round requested successfully".to_string(),
        }),
        Err(e) => HttpResponse::InternalServerError().json(JsonResponse {
            response: format!("Failed to request new E3 round: {}", e),
        }),
    }
}

async fn get_rounds()-> impl Responder {
    match get_e3_round().await {
        Ok(round_count) => {
            let count = RoundCount { round_count: round_count as u32 };
            info!("round_count: {}", count.round_count);
            HttpResponse::Ok().json(count)
        }
        Err(e) => {
            info!("Failed to retrieve round count: {}", e);
            HttpResponse::InternalServerError().body(format!("Error: {}", e))
        }
    }
}

async fn get_ct_by_round(
    data: web::Json<CTRequest>,
) -> impl Responder {
    let mut incoming = data.into_inner();
    info!("Request for round {:?} ciphertext", incoming.round_id);
    let (state_data, _) = get_e3(incoming.round_id).await.unwrap();
    incoming.ct_bytes = state_data.ciphertext_output;
    HttpResponse::Ok().json(incoming)
}

async fn get_pk_by_round(
    data: web::Json<PKRequest>,
) -> impl Responder {
    let mut incoming = data.into_inner();
    info!("Request for round {:?} pk", incoming.round_id);
    let (state_data, _) = get_e3(incoming.round_id).await.unwrap();
    incoming.pk_bytes = state_data.committee_public_key;
    HttpResponse::Ok().json(incoming)
}



pub async fn initialize_crisp_round() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("Starting new CRISP round!");
    
    let contract = EnclaveContract::new(CONFIG.enclave_address.clone()).await?;
    let e3_program: Address = CONFIG.e3_program_address.parse()?;

    // Enable E3 Program
    info!("Enabling E3 Program...");
    match contract.is_e3_program_enabled(e3_program).await {    
        Ok(enabled) => {
            if !enabled {
                match contract.enable_e3_program(e3_program).await {
                    Ok(res) => println!("E3 Program enabled. TxHash: {:?}", res.transaction_hash),
                    Err(e) => println!("Error enabling E3 Program: {:?}", e),
                }
            } else {
                info!("E3 Program already enabled");
            }
        }
        Err(e) => println!("Error checking E3 Program enabled: {:?}", e),
    }

    info!("Generating parameters...");
    let params = generate_bfv_parameters().unwrap().to_bytes();

    info!("Requesting E3...");
    let filter: Address = CONFIG.naive_registry_filter_address.parse()?;
    let threshold: [u32; 2] = [1, 2];
    let start_window: [U256; 2] = [U256::from(Utc::now().timestamp()), U256::from(Utc::now().timestamp() + 600)];
    let duration: U256 = U256::from(600);
    let e3_params = Bytes::from(params);
    let compute_provider_params = Bytes::from(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let res = contract.request_e3(filter, threshold, start_window, duration, e3_program, e3_params, compute_provider_params).await?;
    println!("E3 request sent. TxHash: {:?}", res.transaction_hash);

    Ok(())
}

fn generate_bfv_parameters(
) -> Result<std::sync::Arc<fhe::bfv::BfvParameters>, Box<dyn std::error::Error + Send + Sync>> {
    let degree = 2048;
    let plaintext_modulus: u64 = 1032193;
    let moduli = vec![0x3FFFFFFF000001];

    Ok(BfvParametersBuilder::new()
        .set_degree(degree)
        .set_plaintext_modulus(plaintext_modulus)
        .set_moduli(&moduli)
        .build_arc()?)
}