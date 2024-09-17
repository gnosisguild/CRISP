use hyper::{Request, Method};
use http_body_util::BodyExt;
use serde::{Deserialize, Serialize};
use log::info;
use crate::cli::HyperClientPost;

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct AuthenticationLogin {
    pub postId: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthenticationResponse {
    pub response: String,
    pub jwt_token: String,
}

pub async fn authenticate_user(config: &super::CrispConfig, client: &HyperClientPost) -> Result<AuthenticationResponse, Box<dyn std::error::Error + Send + Sync>> {
    let user = AuthenticationLogin {
        postId: config.authentication_id.clone(),
    };
    
    let body = serde_json::to_string(&user)?;
    let url = format!("{}/authentication_login", config.enclave_address);
    
    let req = Request::builder()
        .header("Content-Type", "application/json")
        .method(Method::POST)
        .uri(url)
        .body(body)?;

    let resp = client.request(req).await?;
    let body_bytes = resp.collect().await?.to_bytes();
    let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
    let auth_res: AuthenticationResponse = serde_json::from_str(&body_str).expect("JSON was not well-formatted");

    info!("Authentication response {:?}", auth_res);
    Ok(auth_res)
}
