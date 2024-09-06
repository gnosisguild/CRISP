use actix_web::{web, HttpResponse, Responder};
use log::info;

use crate::enclave_server::models::{GetRoundRequest, WebResultRequest, AllWebStates, StateLite, StateWeb};
use crate::enclave_server::database::{get_state, get_round_count};

pub fn setup_routes(config: &mut web::ServiceConfig) {
    config
        .route("/get_web_result_all", web::get().to(get_web_result_all))
        .route("/get_round_state_lite", web::post().to(get_round_state_lite))
        .route("/get_round_state", web::post().to(get_round_state))
        .route("/get_web_result", web::post().to(get_web_result))
        .route("/get_round_state_web", web::post().to(get_round_state_web));
}

async fn get_web_result(data: web::Json<GetRoundRequest>) -> impl Responder {
    let incoming = data.into_inner();
    info!("Request web state for round {}", incoming.round_id);

    let (state, _key) = get_state(incoming.round_id);
    
    let response = WebResultRequest {
        round_id: incoming.round_id,
        option_1_tally: state.votes_option_1,
        option_2_tally: state.votes_option_2,
        total_votes: state.votes_option_1 + state.votes_option_2,
        option_1_emoji: state.emojis[0].clone(),
        option_2_emoji: state.emojis[1].clone(),
        end_time: state.start_time + state.poll_length as i64,
    };

    HttpResponse::Ok().json(response)
}

async fn get_web_result_all() -> impl Responder {
    info!("Request all web state.");

    let round_count = get_round_count();
    let states: Vec<WebResultRequest> = (1..round_count)
        .map(|i| {
            let (state, _key) = get_state(i);
            WebResultRequest {
                round_id: i,
                option_1_tally: state.votes_option_1,
                option_2_tally: state.votes_option_2,
                total_votes: state.votes_option_1 + state.votes_option_2,
                option_1_emoji: state.emojis[0].clone(),
                option_2_emoji: state.emojis[1].clone(),
                end_time: state.start_time + state.poll_length as i64,
            }
        })
        .collect();

    let response = AllWebStates { states };
    HttpResponse::Ok().json(response)
}

async fn get_round_state(data: web::Json<GetRoundRequest>) -> impl Responder {
    let incoming = data.into_inner();
    info!("Request state for round {}", incoming.round_id);

    let (state, _key) = get_state(incoming.round_id);
    HttpResponse::Ok().json(state)
}

async fn get_round_state_web(data: web::Json<GetRoundRequest>) -> impl Responder {
    let incoming = data.into_inner();
    info!("Request state for round {}", incoming.round_id);

    let (state, _key) = get_state(incoming.round_id);
    let state_web = StateWeb {
        id: state.id,
        status: state.status,
        poll_length: state.poll_length,
        voting_address: state.voting_address,
        chain_id: state.chain_id,
        ciphernode_count: state.ciphernode_count,
        pk_share_count: state.pk_share_count,
        sks_share_count: state.sks_share_count,
        vote_count: state.vote_count,
        start_time: state.start_time,
        ciphernode_total: state.ciphernode_total,
        emojis: state.emojis,
    };

    HttpResponse::Ok().json(state_web)
}

async fn get_round_state_lite(data: web::Json<GetRoundRequest>) -> impl Responder {
    let incoming = data.into_inner();
    info!("Request state for round {}", incoming.round_id);

    let (state, _key) = get_state(incoming.round_id);
    let state_lite = StateLite {
        id: state.id,
        status: state.status,
        poll_length: state.poll_length,
        voting_address: state.voting_address,
        chain_id: state.chain_id,
        ciphernode_count: state.ciphernode_count,
        pk_share_count: state.pk_share_count,
        sks_share_count: state.sks_share_count,
        vote_count: state.vote_count,
        crp: state.crp,
        pk: state.pk,
        start_time: state.start_time,
        block_start: state.block_start,
        ciphernode_total: state.ciphernode_total,
        emojis: state.emojis,
    };

    HttpResponse::Ok().json(state_lite)
}