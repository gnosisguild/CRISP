use actix_web::{web, HttpResponse, Responder};

use crate::enclave_server::models::JsonResponse;

pub fn setup_routes(config: &mut web::ServiceConfig) {
    config
        .route("/", web::get().to(index_handler))
        .route("/health", web::get().to(health_handler));
}

async fn index_handler() -> impl Responder {
    HttpResponse::Ok().json(JsonResponse {
        response: "Welcome to the enclave server!".to_string(),
    })
}

async fn health_handler() -> impl Responder {
    HttpResponse::Ok().finish()
}
