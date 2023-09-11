use std::net::SocketAddr;

use crate::api_core::{
    bean::{request::ds_user_req::UserLoginInfoReq, response::ds_user_res::UserLoginInfoRes},
    core_results::results::ApiResult,
    core_status::app_status::AppStatus,
    service::login_service::login_service,
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
        return ApiResult::<UserLoginInfoRes>::new_with_err_extra(
            None,
            AppStatus::UserNameNull,
            Some(vec!["用户名为空".to_string()]),
        )
        .into_response();
    }

    let ip = addr.ip().to_string();
    if ip.is_empty() {
        return ApiResult::<UserLoginInfoRes>::new_with_err_extra(
            None,
            AppStatus::IpIsEmpty,
            Some(vec!["ip地址为空".to_string()]),
        )
        .into_response();
    }
    // todo!()
    let res = login_service(user_name, user_password, ip).await;
    match res {
        (Some(data), status) => ApiResult::<UserLoginInfoRes>::new_with_err_extra(
            Some(data),
            status,
            Some(vec!["登录成功".to_string()]),
        )
        .into_response(),
        (_, status) => ApiResult::<UserLoginInfoRes>::new_with_err_extra(
            None,
            status,
            Some(vec!["登录失败".to_string()]),
        )
        .into_response(),
    }
    //     .into_response()
    // let u = UserLoginInfoReq::new();
    // info!("获取数据为{u:?}");
    // Ok(Json(u))
}
