use std::net::SocketAddr;

use api_core::{
    bean::request::ds_user_req::UserLoginInfoReq,
    core_results::results::ApiResult,
    core_status::app_status::AppStatus, service::login_service::login_service,
};
use axum::{
    extract::ConnectInfo,
    response::{IntoResponse, Response},
    Json,
};

pub async fn user_login(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Json(payload): Json<UserLoginInfoReq>,
) -> Response {
    let user_name = payload.user_name.clone();
    let user_password = payload.user_password.clone();
    if user_name.is_empty() {
        return ApiResult::<()>::new_with_err_extra(
            None,
            AppStatus::UserNameNull,
            Some(vec!["用户名为空".to_string()]),
        )
        .into_response();
    }

    let ip = addr.ip().to_string();
    if ip.is_empty() {
        return ApiResult::<()>::new_with_err_extra(
            None,
            AppStatus::IpIsEmpty,
            Some(vec!["ip地址为空".to_string()]),
        )
        .into_response();
    }

    login_service(user_name, user_password, ip)
        .await
        .into_response()
    // let u = UserLoginInfoReq::new();
    // info!("获取数据为{u:?}");
    // Ok(Json(u))
}
