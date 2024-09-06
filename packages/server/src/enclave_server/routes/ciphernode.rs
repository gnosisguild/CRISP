use std::str;
use chrono::Utc;
use fhe::{
    bfv::{BfvParametersBuilder, PublicKey},
    mbfv::{AggregateIter, CommonRandomPoly, PublicKeyShare},
};
use fhe_traits::Serialize as FheSerialize;
use crate::util::timeit::timeit;
use actix_web::{web, HttpResponse, Responder};
use std::io::Read;
use log::info;

use crate::enclave_server::models::{Round, AppState, Ciphernode, JsonResponse, JsonRequest, RegisterNodeResponse, SKSShareRequest, SKSSharePoll, SKSShareResponse, PKShareCount, PKRequest, GetCiphernode, GetEligibilityRequest, CRPRequest};
use crate::enclave_server::database::{get_state, pick_response};

pub fn setup_routes(config: &mut web::ServiceConfig) {
    config
        .route("/register_ciphernode", web::post().to(register_ciphernode))
        .route("/get_pk_share_count", web::post().to(get_pk_share_count))
        .route("/get_pk_by_round", web::post().to(get_pk_by_round))
        .route("/register_sks_share", web::post().to(register_sks_share))
        .route("/get_sks_shares", web::post().to(get_sks_shares))
        .route("/get_crp_by_round", web::post().to(get_crp_by_round))
        .route("/get_node_by_round", web::post().to(get_node_by_round))
        .route("/get_round_eligibility", web::post().to(get_round_eligibility));
}

async fn register_ciphernode(
    state: web::Data<AppState>,
    data: web::Json<JsonRequest>,
) -> impl Responder {
    let incoming = data.into_inner();
    info!("{:?}", incoming.response);
    info!("ID: {:?}", incoming.id);
    info!("Round ID: {:?}", incoming.round_id);

    let (mut state_data, key) = get_state(incoming.round_id);  // Use shared DB

    state_data.pk_share_count += 1;
    state_data.ciphernode_count += 1;

    let cnode = Ciphernode {
        id: incoming.id,
        pk_share: incoming.pk_share,
        sks_share: vec![0],
    };
    state_data.ciphernodes.push(cnode);
    let state_str = serde_json::to_string(&state_data).unwrap();
    state.db.insert(key, state_str.into_bytes()).unwrap();

    info!("pk share store for node id {:?}", incoming.id);
    info!("ciphernode count {:?}", state_data.ciphernode_count);
    info!("ciphernode total {:?}", state_data.ciphernode_total);
    info!("pk share count {:?}", state_data.pk_share_count);

    // Trigger aggregate_pk_shares when all shares are received
    if state_data.ciphernode_count == state_data.ciphernode_total {
        info!("All shares received");
        let _ = aggregate_pk_shares(incoming.round_id, state.clone()).await;  // Share state in aggregation
    }

    let response = RegisterNodeResponse {
        response: "Node Registered".to_string(),
        node_index: state_data.ciphernode_count,
    };

    HttpResponse::Ok().json(response)
}

// Register SKS Share
async fn register_sks_share(
    state: web::Data<AppState>,
    data: web::Json<SKSShareRequest>,
) -> impl Responder {
    let incoming = data.into_inner();
    info!("{:?}", incoming.response);
    info!("Index: {:?}", incoming.index);
    info!("Round ID: {:?}", incoming.round_id);

    let mut round_key = format!("{}-storage", incoming.round_id);
    info!("Database key is {:?}", round_key);

    let state_out = state.db.get(&round_key).unwrap().unwrap();
    let state_out_str = str::from_utf8(&state_out).unwrap();
    let mut state_out_struct: Round = serde_json::from_str(&state_out_str).unwrap();

    state_out_struct.sks_share_count += 1;
    let index = incoming.index;
    state_out_struct.ciphernodes[index as usize].sks_share = incoming.sks_share;

    let state_str = serde_json::to_string(&state_out_struct).unwrap();
    state.db.insert(round_key, state_str.into_bytes()).unwrap();
    info!("sks share stored for node index {:?}", incoming.index);

    // Check if all SKS shares have been received
    if state_out_struct.sks_share_count == state_out_struct.ciphernode_total {
        info!("All sks shares received");
        // TODO: Trigger aggregate_pk_shares or notify cipher nodes
    }

    HttpResponse::Ok().json(JsonResponse { response: pick_response() })
}

// Get SKS Shares
async fn get_sks_shares(
    state: web::Data<AppState>,  
    data: web::Json<SKSSharePoll>,
) -> impl Responder {
    let incoming = data.into_inner();
    let (mut state_data, key) = get_state(incoming.round_id);
    let mut shares = Vec::with_capacity(incoming.ciphernode_count as usize);

    // Check if all SKS shares have been received
    if state_data.sks_share_count == state_data.ciphernode_total {
        info!("All sks shares received... sending to cipher nodes");

        for i in 1..=state_data.ciphernode_total {
            shares.push(state_data.ciphernodes[i as usize].sks_share.clone());
        }

        let response = SKSShareResponse {
            response: "final".to_string(),
            round_id: incoming.round_id,
            sks_shares: shares,
        };
        state_data.status = "Finalized".to_string();
        state.db.insert(key, serde_json::to_string(&state_data).unwrap().into_bytes()).unwrap();
        HttpResponse::Ok().json(response)
    } else {
        let response = SKSShareResponse {
            response: "waiting".to_string(),
            round_id: incoming.round_id,
            sks_shares: shares,
        };
        HttpResponse::Ok().json(response)
    }
}

// Get CRP by Round
async fn get_crp_by_round(
    data: web::Json<CRPRequest>,
) -> impl Responder {
    let mut incoming = data.into_inner();
    info!("Request crp for round {:?}", incoming.round_id);

    let (state_data, _) = get_state(incoming.round_id);
    incoming.crp_bytes = state_data.crp;

    HttpResponse::Ok().json(incoming)
}

// Get PK by Round
async fn get_pk_by_round(
    data: web::Json<PKRequest>,
) -> impl Responder {
    let mut incoming = data.into_inner();
    let (state_data, _) = get_state(incoming.round_id);
    incoming.pk_bytes = state_data.pk;
    info!("Request for round {:?} public key", incoming.round_id);

    HttpResponse::Ok().json(incoming)
}

// Get PK Share Count
async fn get_pk_share_count(
    data: web::Json<PKShareCount>,
) -> impl Responder {
    let mut incoming = data.into_inner();
    let (state_data, _) = get_state(incoming.round_id);
    incoming.share_id = state_data.pk_share_count;

    HttpResponse::Ok().json(incoming)
}

// Get Round Eligibility
async fn get_round_eligibility(
    data: web::Json<GetEligibilityRequest>,
) -> impl Responder {
    let mut incoming = data.into_inner();
    info!("Request node eligibility for round {:?}", incoming.round_id);

    let (state_data, _) = get_state(incoming.round_id);

    for i in 1..state_data.ciphernodes.len() {
        info!("checking ciphernode {:?}", i);
        if state_data.ciphernodes[i].id == incoming.node_id {
            incoming.is_eligible = true;
            incoming.reason = "Previously Registered".to_string();
        }
    }

    if state_data.ciphernode_total == state_data.ciphernode_count && incoming.reason != "Previously Registered" {
        incoming.is_eligible = false;
        incoming.reason = "Round Full".to_string();
    }

    if state_data.ciphernode_total > state_data.ciphernode_count && incoming.reason != "Previously Registered" {
        incoming.is_eligible = true;
        incoming.reason = "Open Node Spot".to_string();
    }

    let timestamp = Utc::now().timestamp();
    if timestamp >= (state_data.start_time + state_data.poll_length as i64) {
        incoming.is_eligible = false;
        incoming.reason = "Waiting For New Round".to_string();
    }

    HttpResponse::Ok().json(incoming)
}

// Get Node by Round
async fn get_node_by_round(
    data: web::Json<GetCiphernode>,
) -> impl Responder {
    let incoming = data.into_inner();
    info!("Request node data for round {:?}", incoming.round_id);

    let (state_data, _) = get_state(incoming.round_id);
    let mut cnode = Ciphernode {
        id: 0,
        pk_share: vec![0],
        sks_share: vec![0],
    };

    for i in 0..state_data.ciphernodes.len() {
        if state_data.ciphernodes[i].id == incoming.ciphernode_id {
            cnode.id = state_data.ciphernodes[i].id;
            cnode.pk_share = state_data.ciphernodes[i].pk_share.clone();
            cnode.sks_share = state_data.ciphernodes[i].sks_share.clone();
        }
    }

    if cnode.id != 0 {
        HttpResponse::Ok().json(cnode)
    } else {
        HttpResponse::Ok().json(JsonResponse {
            response: "Ciphernode Not Registered".to_string(),
        })
    }
}

// Aggregate PK Shares (async)
async fn aggregate_pk_shares(
    round_id: u32,
    state: web::Data<AppState>,  // Access shared state
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("aggregating validator keyshare");

    let degree = 4096;
    let plaintext_modulus: u64 = 4096;
    let moduli = vec![0xffffee001, 0xffffc4001, 0x1ffffe0001];

    // Generate BFV parameters
    let params = timeit!(
        "Parameters generation",
        BfvParametersBuilder::new()
            .set_degree(degree)
            .set_plaintext_modulus(plaintext_modulus)
            .set_moduli(&moduli)
            .build_arc()?
    );

    let (mut state_data, round_key) = get_state(round_id);

    let crp = CommonRandomPoly::deserialize(&state_data.crp, &params)?;

    struct Party {
        pk_share: PublicKeyShare,
    }

    let mut parties: Vec<Party> = Vec::new();
    for i in 1..=state_data.ciphernode_total {
        info!("Aggregating PKShare... id {}", i);
        let data_des = PublicKeyShare::deserialize(&state_data.ciphernodes[i as usize].pk_share, &params, crp.clone()).unwrap();
        parties.push(Party { pk_share: data_des });
    }

    let pk = timeit!("Public key aggregation", {
        let pk: PublicKey = parties.iter().map(|p| p.pk_share.clone()).aggregate()?;
        pk
    });

    info!("Multiparty Public Key Generated");
    state_data.pk = pk.to_bytes();

    state.db.insert(round_key, serde_json::to_string(&state_data).unwrap().into_bytes()).unwrap();
    info!("aggregate pk stored for round {:?}", round_id);

    Ok(())
}