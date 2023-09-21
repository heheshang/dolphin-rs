use std::net::SocketAddr;

use crate::{
    cypt::security::get_authenticator,
    web::bean::{request::ds_user_req::UserLoginInfoReq},
};
use axum::{
    extract::ConnectInfo,
    routing::post,
    Json,
    Router,
};
use dolphin_common::{core_error::error::Error, core_results::results::Result};
use serde_json::{json, Value};
use tracing::error;


pub fn routes() -> Router {
    Router::new()
        .route("/api/login", post(api_login_handler))
        .route("/api/logoff", post(api_logoff_handler))
}

pub async fn api_login_handler(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Json(payload): Json<UserLoginInfoReq>,
) -> Result<Json<Value>> {
    let user_name = payload.user_name.clone();
    let user_password = payload.user_password.clone();
    if user_name.is_empty() {
        return Err(Error::UserNamePasswdError);
    }
    // let ip = "127.0.0.1".to_string();
    let ip = addr.ip().to_string();
    if ip.is_empty() {
        return Err(Error::IpIsEmpty);
    }
    get_authenticator()
        .authenticate(user_name, user_password, ip)
        .await
        .map(|res| {
            let session_id = res.get("session_id").unwrap();
            Json(json!({
            "session_id": session_id.to_string(),
            }))
        })
        .map_err(|e| {
            error!("api_login_handler error: {:?}", e);
            e
        })
}
pub async fn api_logoff_handler() {}
