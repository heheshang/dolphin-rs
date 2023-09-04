use std::net::SocketAddr;

use api_core::{
    bean::request::ds_user_req::UserLoginInfoReq,
    core_results::results::ApiResult,
    core_status::app_status::AppStatus,
};
use axum::{
    extract::ConnectInfo,
    response::{IntoResponse, Response},
    Json,
};

pub async fn login(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Json(payload): Json<UserLoginInfoReq>,
) -> Response {
    let user_name = payload.user_name.clone();
    if user_name.is_empty() {
        return ApiResult::<()>::new_with_err_extra(
            Some(()),
            AppStatus::UserNameNull,
            Some(vec!["用户名为空".to_string()]),
        )
        .into_response();
    }

    let ip = addr.ip().to_string();
    if ip.is_empty() {
        return ApiResult::<()>::new_with_err_extra(
            Some(()),
            AppStatus::IpIsEmpty,
            Some(vec!["ip地址为空".to_string()]),
        )
        .into_response();
    }

    let res = payload.login().await.into_response();
    res
    // info!("获取数据为{u:?}");
    // Ok(Json(u))
}
