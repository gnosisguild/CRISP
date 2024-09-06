use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use router::Router;
use std::io::Read;
use log::info;

use crate::enclave_server::models::{GetRoundRequest, WebResultRequest, AllWebStates, StateLite, StateWeb};
use crate::enclave_server::database::{get_state, get_round_count};


pub fn setup_routes(router: &mut Router) {
    router.get("/get_web_result_all", get_web_result_all, "get_web_result_all");
    router.post("/get_round_state_lite", get_round_state_lite, "get_round_state_lite");
    router.post("/get_round_state", get_round_state, "get_round_state");
    router.post("/get_web_result", get_web_result, "get_web_result");
    router.post("/get_round_state_web", get_round_state_web, "get_round_state_web");
}

fn get_web_result(req: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();
    // read the POST body
    req.body.read_to_string(&mut payload).unwrap();
    let incoming: GetRoundRequest = serde_json::from_str(&payload).unwrap();
    info!("Request web state for round {:?}", incoming.round_id);

    let (state, _key) = get_state(incoming.round_id);
    
    let response = WebResultRequest {
        round_id: incoming.round_id,
        option_1_tally: state.votes_option_1,
        option_2_tally: state.votes_option_2,
        total_votes: state.votes_option_1 + state.votes_option_2,
        option_1_emoji: state.emojis[0].clone(),
        option_2_emoji: state.emojis[1].clone(),
        end_time: state.start_time + state.poll_length as i64
    };

    let out = serde_json::to_string(&response).unwrap();

    let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok, out)))
}

fn get_web_result_all(_req: &mut Request) -> IronResult<Response> {
    info!("Request all web state.");

    let round_count = get_round_count();
    let mut states: Vec<WebResultRequest> = Vec::with_capacity(round_count as usize);

    for i in 1..round_count {
        let (state, _key) = get_state(i);
        let web_state = WebResultRequest {
            round_id: i,
            option_1_tally: state.votes_option_1,
            option_2_tally: state.votes_option_2,
            total_votes: state.votes_option_1 + state.votes_option_2,
            option_1_emoji: state.emojis[0].clone(),
            option_2_emoji: state.emojis[1].clone(),
            end_time: state.start_time + state.poll_length as i64
        };
        states.push(web_state);
    }

    let response = AllWebStates { states: states };
    let out = serde_json::to_string(&response).unwrap();

    let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok, out)))
}



fn get_round_state(req: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();
    // read the POST body
    req.body.read_to_string(&mut payload).unwrap();
    let incoming: GetRoundRequest = serde_json::from_str(&payload).unwrap();
    info!("Request state for round {:?}", incoming.round_id);

    let (state, _key) = get_state(incoming.round_id);
    let out = serde_json::to_string(&state).unwrap();

    let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok, out)))
}

fn get_round_state_web(req: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();
    // read the POST body
    req.body.read_to_string(&mut payload).unwrap();
    let incoming: GetRoundRequest = serde_json::from_str(&payload).unwrap();
    info!("Request state for round {:?}", incoming.round_id);

    let (state, _key) = get_state(incoming.round_id);
    let state_lite = StateWeb {
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
        ciphernode_total:  state.ciphernode_total,
        emojis: state.emojis,
    };

    let out = serde_json::to_string(&state_lite).unwrap();

    let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok, out)))
}


fn get_round_state_lite(req: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();
    // read the POST body
    req.body.read_to_string(&mut payload).unwrap();
    let incoming: GetRoundRequest = serde_json::from_str(&payload).unwrap();
    info!("Request state for round {:?}", incoming.round_id);

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
        ciphernode_total:  state.ciphernode_total,
        emojis: state.emojis,
    };

    let out = serde_json::to_string(&state_lite).unwrap();

    let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok, out)))
}
