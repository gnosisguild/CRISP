use actix_web::{web, HttpResponse, Responder};
use log::info;

use crate::enclave_server::database::{get_e3, get_e3_round};
use crate::enclave_server::models::{
    AllWebStates, E3StateLite, GetRoundRequest, WebResultRequest,
};

pub fn setup_routes(config: &mut web::ServiceConfig) {
    config
        .route("/get_web_result_all", web::get().to(get_web_result_all))
        .route(
            "/get_round_state_lite",
            web::post().to(get_round_state_lite),
        )
        .route("/get_round_state", web::post().to(get_round_state))
        .route("/get_web_result", web::post().to(get_web_result));
}

async fn get_web_result(data: web::Json<GetRoundRequest>) -> impl Responder {
    let incoming = data.into_inner();
    info!("Request web state for round {}", incoming.round_id);

    let (state, _) = get_e3(incoming.round_id as u64).await.unwrap();

    let response = WebResultRequest {
        round_id: state.id,
        option_1_tally: state.votes_option_1,
        option_2_tally: state.votes_option_2,
        total_votes: state.votes_option_1 + state.votes_option_2,
        option_1_emoji: state.emojis[0].clone(),
        option_2_emoji: state.emojis[1].clone(),
        end_time: state.expiration,
    };

    HttpResponse::Ok().json(response)
}

async fn get_web_result_all() -> impl Responder {
    info!("Request all web state.");

    let round_count = match get_e3_round().await {
        Ok(count) => count,
        Err(e) => {
            info!("Error retrieving round count: {:?}", e);
            return HttpResponse::InternalServerError().body("Failed to retrieve round count");
        }
    };

    let mut states = Vec::new();
    for i in 1..round_count {
        match get_e3(i).await {
            Ok((state, _key)) => {
                let web_result = WebResultRequest {
                    round_id: i,
                    option_1_tally: state.votes_option_1,
                    option_2_tally: state.votes_option_2,
                    total_votes: state.votes_option_1 + state.votes_option_2,
                    option_1_emoji: state.emojis[0].clone(),
                    option_2_emoji: state.emojis[1].clone(),
                    end_time: state.expiration,
                };
                states.push(web_result);
            }
            Err(e) => {
                info!("Error retrieving state for round {}: {:?}", i, e);
                return HttpResponse::InternalServerError()
                    .body(format!("Failed to retrieve state for round {}", i));
            }
        }
    }

    let response = AllWebStates { states };
    HttpResponse::Ok().json(response)
}

async fn get_round_state(data: web::Json<GetRoundRequest>) -> impl Responder {
    let incoming = data.into_inner();
    info!("Request state for round {}", incoming.round_id);

    let (state, _key) = get_e3(incoming.round_id as u64).await.unwrap();
    HttpResponse::Ok().json(state)
}

async fn get_round_state_lite(data: web::Json<GetRoundRequest>) -> impl Responder {
    let incoming = data.into_inner();
    info!("Request state for round {}", incoming.round_id);

    match get_e3(incoming.round_id as u64).await {
        Ok((state, _)) => {
            let state_lite = E3StateLite {
                id: state.id,
                chain_id: state.chain_id,
                enclave_address: state.enclave_address,

                status: state.status,
                vote_count: state.vote_count,

                start_time: state.start_time,
                duration: state.duration,
                expiration: state.expiration,

                committee_public_key: state.committee_public_key,
                emojis: state.emojis,
            };
            HttpResponse::Ok().json(state_lite)
        }
        Err(e) => {
            info!("Error getting E3 state: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to get E3 state")
        }
    }
}
