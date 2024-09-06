use std::str;
use iron::mime::Mime;
use iron::prelude::*;
use iron::status;
use router::Router;
use std::io::Read;
use jwt::SignWithKey;
use sha2::Sha256;
use std::collections::BTreeMap;
use hmac::{Hmac, Mac};
use log::info;

use crate::enclave_server::models::{JsonResponse, AuthenticationLogin, AuthenticationDB, AuthenticationResponse};
use crate::enclave_server::database::{GLOBAL_DB, pick_response};

pub fn setup_routes(router: &mut Router) {
    router.get("/", handler, "index");
    router.get("/health", health_handler, "health");
    router.post(
        "/authentication_login",
        authentication_login,
        "authentication_login",
    );
}

fn handler(_req: &mut Request) -> IronResult<Response> {
    let response = JsonResponse {
        response: pick_response(),
    };
    let out = serde_json::to_string(&response).unwrap();
    info!("index handler hit");
    let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok, out)))
}

fn health_handler(_req: &mut Request) -> IronResult<Response> {
    let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok)))
}

fn authentication_login(req: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();
    // read the POST body
    req.body.read_to_string(&mut payload).unwrap();
    let incoming: AuthenticationLogin = serde_json::from_str(&payload).unwrap();
    info!("Twitter Login Request");

    // hmac
    let hmac_key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret").unwrap();
    let mut claims = BTreeMap::new();
    claims.insert("postId", incoming.postId);
    let token_str = claims.sign_with_key(&hmac_key).unwrap();

    // db
    let key = "authentication";
    let mut authsdb = GLOBAL_DB.get(key).unwrap();
    let mut response_str = "".to_string();
    let mut jwt_token = "".to_string();

    if authsdb == None {
        info!("initializing first auth in db");
        // hmac
        let auth_struct = AuthenticationDB {
            jwt_tokens: vec![token_str.clone()],
        };
        let authsdb_str = serde_json::to_string(&auth_struct).unwrap();
        let authsdb_bytes = authsdb_str.into_bytes();
        GLOBAL_DB.insert(key, authsdb_bytes).unwrap();
        // set response
        response_str = "Authorized".to_string();
    } else {
        // look for previous auth
        let mut au_db = authsdb.unwrap();
        let authsdb_out_str = str::from_utf8(&au_db).unwrap();
        let mut authsdb_out_struct: AuthenticationDB = serde_json::from_str(&authsdb_out_str).unwrap();

        for i in 0..authsdb_out_struct.jwt_tokens.len() {
            if authsdb_out_struct.jwt_tokens[i as usize] == token_str {
                info!("Found previous login.");
                response_str = "Already Authorized".to_string();
            }
        };

        if response_str != "Already Authorized" {
            info!("Inserting new login to db.");
            authsdb_out_struct.jwt_tokens.push(token_str.clone());
            let authsdb_str = serde_json::to_string(&authsdb_out_struct).unwrap();
            let authsdb_bytes = authsdb_str.into_bytes();
            GLOBAL_DB.insert(key, authsdb_bytes).unwrap();
            response_str = "Authorized".to_string();
        }
    };

    let response = AuthenticationResponse {
        response: response_str,
        jwt_token: token_str,
    };
    let out = serde_json::to_string(&response).unwrap();

    let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok, out)))
}

