use std::str;
use chrono::Utc;
use fhe::{
    bfv::{BfvParametersBuilder, PublicKey},
    mbfv::{AggregateIter, CommonRandomPoly, PublicKeyShare},
};
use fhe_traits::Serialize as FheSerialize;
use crate::util::timeit::timeit;

use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use router::Router;
use std::io::Read;
use log::info;

use crate::enclave_server::models::{Round, Ciphernode, JsonResponse, JsonRequest, RegisterNodeResponse, SKSShareRequest, SKSSharePoll, SKSShareResponse, PKShareCount, PKRequest, GetCiphernode, GetEligibilityRequest, CRPRequest};
use crate::enclave_server::database::{GLOBAL_DB, get_state, pick_response};

pub fn setup_routes(router: &mut Router) {
    router.post("/register_ciphernode", register_ciphernode, "register_ciphernode");
    router.post("/get_pk_share_count", get_pk_share_count, "get_pk_share_count");
    router.post("/get_pk_by_round", get_pk_by_round, "get_pk_by_round");
    router.post("/register_sks_share", register_sks_share, "register_sks_share");
    router.post("/get_sks_shares", get_sks_shares, "get_sks_shares");
    router.post("/get_crp_by_round", get_crp_by_round, "get_crp_by_round");
    router.post("/get_node_by_round", get_node_by_round, "get_node_by_round");
    router.post("/get_round_eligibility", get_round_eligibility, "get_round_eligibility");
}

#[tokio::main]
async fn register_ciphernode(req: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();

    // read the POST body
    req.body.read_to_string(&mut payload).unwrap();

    // we're expecting the POST to match the format of our JsonRequest struct
    let incoming: JsonRequest = serde_json::from_str(&payload).unwrap();
    info!("{:?}", incoming.response);
    info!("ID: {:?}", incoming.id);
    info!("Round ID: {:?}", incoming.round_id);

    let (mut state, key) = get_state(incoming.round_id);

    state.pk_share_count = state.pk_share_count + 1;
    state.ciphernode_count = state.ciphernode_count + 1;
    let cnode = Ciphernode {
        id: incoming.id,
        pk_share: incoming.pk_share,
        sks_share: vec![0],
    };
    state.ciphernodes.push(cnode);
    let state_str = serde_json::to_string(&state).unwrap();
    let state_bytes = state_str.into_bytes();
    GLOBAL_DB.insert(key, state_bytes).unwrap();

    info!("pk share store for node id {:?}", incoming.id);
    info!("ciphernode count {:?}", state.ciphernode_count);
    info!("ciphernode total {:?}", state.ciphernode_total);
    info!("pk share count {:?}", state.pk_share_count);

    if state.ciphernode_count == state.ciphernode_total {
        info!("All shares received");
        let _ = aggregate_pk_shares(incoming.round_id).await;
    }

    let response = RegisterNodeResponse {
        response: "Node Registered".to_string(),
        node_index: state.ciphernode_count,
        };
    let out = serde_json::to_string(&response).unwrap();

    let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok, out)))
}

fn register_sks_share(req: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();

    // read the POST body
    req.body.read_to_string(&mut payload).unwrap();

    // we're expecting the POST to match the format of our JsonRequest struct
    let incoming: SKSShareRequest = serde_json::from_str(&payload).unwrap();
    info!("{:?}", incoming.response);
    info!("Index: {:?}", incoming.index); // cipher node id (based on first upload of pk share)
    info!("Round ID: {:?}", incoming.round_id);


    let mut round_key = incoming.round_id.to_string();
    round_key.push_str("-storage");
    info!("Database key is {:?}", round_key);

    let state_out = GLOBAL_DB.get(round_key.clone()).unwrap().unwrap();
    let state_out_str = str::from_utf8(&state_out).unwrap();
    let mut state_out_struct: Round = serde_json::from_str(&state_out_str).unwrap();
    state_out_struct.sks_share_count = state_out_struct.sks_share_count + 1;

    let index = incoming.index; // TODO use hashmap with node id as key 
    state_out_struct.ciphernodes[index as usize].sks_share = incoming.sks_share;
    let state_str = serde_json::to_string(&state_out_struct).unwrap();
    let state_bytes = state_str.into_bytes();
    GLOBAL_DB.insert(round_key, state_bytes).unwrap();
    info!("sks share stored for node index {:?}", incoming.index);

    // toso get share threshold from client config
    if state_out_struct.sks_share_count == state_out_struct.ciphernode_total {
        info!("All sks shares received");
        //aggregate_pk_shares(incoming.round_id).await;
        // TODO: maybe notify cipher nodes
    }

    // create a response with our random string, and pass in the string from the POST body
    let response = JsonResponse { response: pick_response() };
    let out = serde_json::to_string(&response).unwrap();

    let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok, out)))
}

fn get_sks_shares(req: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();

    // read the POST body
    req.body.read_to_string(&mut payload).unwrap();

    // we're expecting the POST to match the format of our JsonRequest struct
    let incoming: SKSSharePoll = serde_json::from_str(&payload).unwrap();
    //const length: usize = incoming.cyphernode_count;

    let (mut state, key) = get_state(incoming.round_id);

    let mut shares = Vec::with_capacity(incoming.ciphernode_count as usize);

    // toso get share threshold from client config
    if state.sks_share_count == state.ciphernode_total {
        info!("All sks shares received... sending to cipher nodes");
        for i in 1..state.ciphernode_total + 1 {
            info!("reading share {:?}", i);
            shares.push(state.ciphernodes[i as usize].sks_share.clone());
        }
        let response = SKSShareResponse { 
            response: "final".to_string(),
            round_id: incoming.round_id,
            sks_shares: shares,
        };
        state.status = "Finalized".to_string();
        let state_str = serde_json::to_string(&state).unwrap();
        let state_bytes = state_str.into_bytes();
        GLOBAL_DB.insert(key, state_bytes).unwrap();
        let out = serde_json::to_string(&response).unwrap();
        info!("get rounds hit");

        let content_type = "application/json".parse::<Mime>().unwrap();
        Ok(Response::with((content_type, status::Ok, out)))
    } else {
        let response = SKSShareResponse { 
            response: "waiting".to_string(),
            round_id: incoming.round_id,
            sks_shares: shares,
        };
        let out = serde_json::to_string(&response).unwrap();
        info!("get rounds hit");

        let content_type = "application/json".parse::<Mime>().unwrap();
        Ok(Response::with((content_type, status::Ok, out)))
    }
}

fn get_crp_by_round(req: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();
    // read the POST body
    req.body.read_to_string(&mut payload).unwrap();
    let mut incoming: CRPRequest = serde_json::from_str(&payload).unwrap();
    info!("Request crp for round {:?}", incoming.round_id);

    let (state, _key) = get_state(incoming.round_id);
    incoming.crp_bytes = state.crp;
    let out = serde_json::to_string(&incoming).unwrap();

    let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok, out)))
}

fn get_pk_by_round(req: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();
    // read the POST body
    req.body.read_to_string(&mut payload).unwrap();
    let mut incoming: PKRequest = serde_json::from_str(&payload).unwrap();

    let (state, _key) = get_state(incoming.round_id);
    incoming.pk_bytes = state.pk;
    let out = serde_json::to_string(&incoming).unwrap();

    let content_type = "application/json".parse::<Mime>().unwrap();
    info!("Request for round {:?} public key", incoming.round_id);
    Ok(Response::with((content_type, status::Ok, out)))
}

fn get_pk_share_count(req: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();
    // read the POST body
    req.body.read_to_string(&mut payload).unwrap();

    let mut incoming: PKShareCount = serde_json::from_str(&payload).unwrap();

    let (state, _key) = get_state(incoming.round_id);
    incoming.share_id = state.pk_share_count;
    let out = serde_json::to_string(&incoming).unwrap();

    let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok, out)))
}

fn get_round_eligibility(req: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();
    // read the POST body
    req.body.read_to_string(&mut payload).unwrap();
    let mut incoming: GetEligibilityRequest = serde_json::from_str(&payload).unwrap();
    info!("Request node elegibility for round {:?}", incoming.round_id);

    let (state, _key) = get_state(incoming.round_id);

    for i in 1..state.ciphernodes.len() {
        info!("checking ciphernode {:?}", i);
        info!("server db id {:?}", state.ciphernodes[i as usize].id);
        info!("incoming request id {:?}", incoming.node_id);
        if state.ciphernodes[i as usize].id == incoming.node_id {
            incoming.is_eligible = true;
            incoming.reason = "Previously Registered".to_string();
        };
    };

    if state.ciphernode_total == state.ciphernode_count && incoming.reason != "Previously Registered" {
        incoming.is_eligible = false;
        incoming.reason = "Round Full".to_string();
    };

    if state.ciphernode_total > state.ciphernode_count && incoming.reason != "Previously Registered" {
        incoming.is_eligible = true;
        incoming.reason = "Open Node Spot".to_string();
    };

    let init_time = Utc::now();
    let timestamp = init_time.timestamp();

    if timestamp >= (state.start_time + state.poll_length as i64) {
        incoming.is_eligible = false;
        incoming.reason = "Waiting For New Round".to_string();
    }

    let out = serde_json::to_string(&incoming).unwrap();

    let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok, out)))
}

fn get_node_by_round(req: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();
    // read the POST body
    req.body.read_to_string(&mut payload).unwrap();
    let incoming: GetCiphernode = serde_json::from_str(&payload).unwrap();
    info!("Request node data for round {:?}", incoming.round_id);

    let (state, _key) = get_state(incoming.round_id);
    let mut cnode = Ciphernode {
        id: 0,
        pk_share: vec![0],
        sks_share: vec![0],
    };

    for i in 0..state.ciphernodes.len() {
        if state.ciphernodes[i as usize].id == incoming.ciphernode_id {
            cnode.id = state.ciphernodes[i as usize].id;
            cnode.pk_share = state.ciphernodes[i as usize].pk_share.clone();
            cnode.sks_share = state.ciphernodes[i as usize].sks_share.clone();
        };
    };

    if cnode.id != 0 {
        let out = serde_json::to_string(&cnode).unwrap();

        let content_type = "application/json".parse::<Mime>().unwrap();
        Ok(Response::with((content_type, status::Ok, out)))
    } else {
        let response = JsonResponse { response: "Ciphernode Not Registered".to_string() };
        let out = serde_json::to_string(&response).unwrap();

        let content_type = "application/json".parse::<Mime>().unwrap();
        Ok(Response::with((content_type, status::Ok, out)))
    }

    // let response = JsonResponse { response: "Ciphernode Not Registered".to_string() };
    // let out = serde_json::to_string(&response).unwrap();

    // let content_type = "application/json".parse::<Mime>().unwrap();
    // Ok(Response::with((content_type, status::Ok, out)))
}


async fn aggregate_pk_shares(round_id: u32) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("aggregating validator keyshare");

    let degree = 4096;
    let plaintext_modulus: u64 = 4096;
    let moduli = vec![0xffffee001, 0xffffc4001, 0x1ffffe0001];

    // Generate a deterministic seed for the Common Poly
    //let mut seed = <ChaCha8Rng as SeedableRng>::Seed::default();

    // Let's generate the BFV parameters structure.
    let params = timeit!(
        "Parameters generation",
        BfvParametersBuilder::new()
            .set_degree(degree)
            .set_plaintext_modulus(plaintext_modulus)
            .set_moduli(&moduli)
            .build_arc()?
    );

    let mut round_key = round_id.to_string();
    round_key.push_str("-storage");
    info!("Database key is {:?}", round_key);

    let state_out = GLOBAL_DB.get(round_key.clone()).unwrap().unwrap();
    let state_out_str = str::from_utf8(&state_out).unwrap();
    let mut state: Round = serde_json::from_str(&state_out_str).unwrap();
    info!("checking db after drop {:?}", state.ciphernode_count);
    info!("{:?}", state.ciphernodes[0].id);
    //info!("{:?}", state.ciphernodes[0].pk_share);

    //let crp = CommonRandomPoly::new_deterministic(&params, seed)?;
    let crp = CommonRandomPoly::deserialize(&state.crp, &params)?;

    // Party setup: each party generates a secret key and shares of a collective
    // public key.
    struct Party {
        pk_share: PublicKeyShare,
    }

    let mut parties :Vec<Party> = Vec::new();
    for i in 1..state.ciphernode_total + 1 { // todo fix init code that causes offset
        // read in pk_shares from storage
        info!("Aggregating PKShare... id {}", i);
        let data_des = PublicKeyShare::deserialize(&state.ciphernodes[i as usize].pk_share, &params, crp.clone()).unwrap();
        // let pk_share = PublicKeyShare::new(&sk_share, crp.clone(), &mut thread_rng())?;
        parties.push(Party { pk_share: data_des });
    }

    // Aggregation: this could be one of the parties or a separate entity. Or the
    // parties can aggregate cooperatively, in a tree-like fashion.
    let pk = timeit!("Public key aggregation", {
        let pk: PublicKey = parties.iter().map(|p| p.pk_share.clone()).aggregate()?;
        pk
    });
    //info!("{:?}", pk);
    info!("Multiparty Public Key Generated");
    let store_pk = pk.to_bytes();
    state.pk = store_pk;
    let state_str = serde_json::to_string(&state).unwrap();
    let state_bytes = state_str.into_bytes();
    GLOBAL_DB.insert(round_key, state_bytes).unwrap();
    info!("aggregate pk stored for round {:?}", round_id);
    Ok(())
}
