mod index;
mod auth;
mod state;
mod voting;
mod rounds;
mod ciphernode;

use actix_web::{web, HttpResponse, Responder};

pub fn setup_routes(config: &mut web::ServiceConfig) {
    index::setup_routes(config);
    auth::setup_routes(config);
    state::setup_routes(config);
    voting::setup_routes(config);
    rounds::setup_routes(config);
    ciphernode::setup_routes(config);

}