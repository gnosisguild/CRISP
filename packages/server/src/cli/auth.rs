use hyper::{Request, Method};
use http_body_util::BodyExt;
use serde::{Deserialize, Serialize};
use crate::cli::HyperClientPost;

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthenticationLogin {
    #[allow(non_snake_case)]
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

    let out = serde_json::to_string(&user).unwrap();
    let mut url = config.enclave_address.clone();
    url.push_str("/authentication_login");
    let req = Request::builder()
        .method(Method::POST)
        .uri(url)
        .body(out)?;

    let resp = client.request(req).await?;
    let body_bytes = resp.collect().await?.to_bytes();
    let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
    let auth_res: AuthenticationResponse = serde_json::from_str(&body_str).expect("JSON was not well-formatted");

    println!("Authentication response {:?}", auth_res);
    Ok(auth_res)
}
