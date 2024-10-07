use log::{info, error};
use actix_web::{web, HttpResponse, Responder};
use alloy::primitives::{Address, U256, Bytes};
use chrono::Utc;
use fhe::bfv::BfvParametersBuilder;
use fhe_traits::Serialize;
use crate::server::blockchain::relayer::EnclaveContract;
use crate::server::config::CONFIG;
use crate::server::models::{CTRequest, CurrentRound, AppState, PKRequest, CronRequestE3, JsonResponse};
use crate::server::database::get_e3;

pub fn setup_routes(config: &mut web::ServiceConfig) {
    config
        .service(
            web::scope("/rounds")
                .route("/current", web::get().to(get_current_round))
                .route("/public-key", web::post().to(get_public_key))
                .route("/ciphertext", web::post().to(get_ciphertext))
                .route("/request", web::post().to(request_new_round))
        );
}

/// Request a new E3 round
/// 
/// # Arguments
/// 
/// * `data` - The request data containing the cron API key
/// 
/// # Returns
/// 
/// * A JSON response indicating the success of the operation
async fn request_new_round(
    data: web::Json<CronRequestE3>
) -> impl Responder {
    if data.cron_api_key != CONFIG.cron_api_key {
        return HttpResponse::Unauthorized().json(JsonResponse {
            response: "Invalid API key".to_string(),
        });
    }

    match initialize_crisp_round().await {
        Ok(_) => HttpResponse::Ok().json(JsonResponse {
            response: "New E3 round requested successfully".to_string(),
        }),
        Err(e) => HttpResponse::InternalServerError().json(JsonResponse {
            response: format!("Failed to request new E3 round: {}", e),
        }),
    }
}

/// Get the current E3 round
/// 
/// # Arguments
/// 
/// * `AppState` - The application state
/// 
/// # Returns
/// 
/// * A JSON response containing the current round
async fn get_current_round(state: web::Data<AppState>) -> impl Responder {
    match state.sled.get::<CurrentRound>("e3:current_round").await {
        Ok(Some(current_round)) => HttpResponse::Ok().json(current_round),
        Ok(None) => HttpResponse::NotFound().json(JsonResponse {
            response: "No current round found".to_string(),
        }),
        Err(e) => HttpResponse::InternalServerError().json(JsonResponse {
            response: format!("Failed to retrieve current round: {}", e),
        }),
    }
}

/// Get the ciphertext for a given round
/// 
/// # Arguments
/// 
/// * `CTRequest` - The request data containing the round ID
/// 
/// # Returns
/// 
/// * A JSON response containing the ciphertext
async fn get_ciphertext(
    data: web::Json<CTRequest>,
) -> impl Responder {
    let mut incoming = data.into_inner();

    let (state_data, _) = get_e3(incoming.round_id).await.unwrap();
    
    incoming.ct_bytes = state_data.ciphertext_output;
    
    HttpResponse::Ok().json(incoming)
}

/// Get the public key for a given round
/// 
/// # Arguments
/// 
/// * `PKRequest` - The request data containing the round ID
/// 
/// # Returns
/// 
/// * A JSON response containing the public key
async fn get_public_key(
    data: web::Json<PKRequest>,
) -> impl Responder {
    let mut incoming = data.into_inner();

    let (state_data, _) = get_e3(incoming.round_id).await.unwrap();
    
    incoming.pk_bytes = state_data.committee_public_key;
    
    HttpResponse::Ok().json(incoming)
}

/// Initialize a new CRISP round
/// 
/// Creates a new CRISP round by enabling the E3 program, generating the necessary parameters, and requesting E3.
/// 
/// # Returns
/// 
/// * A result indicating the success of the operation
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
        Err(e) => error!("Error checking E3 Program enabled: {:?}", e),
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
    info!("E3 request sent. TxHash: {:?}", res.transaction_hash);

    Ok(())
}

fn generate_bfv_parameters(
) -> Result<std::sync::Arc<fhe::bfv::BfvParameters>, Box<dyn std::error::Error + Send + Sync>> {
    let degree = 2048;
    let plaintext_modulus: u64 = 1032193;
    let moduli = vec![0xffffffff00001];

    Ok(BfvParametersBuilder::new()
        .set_degree(degree)
        .set_plaintext_modulus(plaintext_modulus)
        .set_moduli(&moduli)
        .build_arc()?)
}