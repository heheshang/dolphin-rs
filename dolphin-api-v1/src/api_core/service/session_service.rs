use crate::api_core::client::service::{_session_client, SESSION_SERVICE};
use anyhow::Result;
use dolphin_common::{
    core_error::error::DolphinErrorInfo,
    core_results::results::ApiResult,
    core_status::app_status::AppStatus,
};
use proto::{
    ds_session::{
        CreateDsSessionRequest,
        DeleteDsSessionRequest,
        DsSession,
        GetDsSessionByIdRequest,
        GetDsSessionUserIdRequest,
        UpdateDsSessionRequest,
    },
    ds_user::DsUser,
};
use tracing::{info, log::error};
#[allow(dead_code)]
pub async fn get_ds_session_by_id(session_id: String) -> ApiResult<DsSession> {
    let client = match client().await {
        Ok(value) => value,
        Err(value) => return value,
    };
    let request = tonic::Request::new(GetDsSessionByIdRequest {
        id: session_id.clone(),
    });
    let response = client.clone().get_ds_session_by_id(request).await;
    match response {
        Ok(res) => {
            let session = res.into_inner();
            ApiResult::build(Some(session))
        }
        Err(_) => ApiResult::new_with_err_status(None, AppStatus::LoginSessionFailed),
    }
}

pub async fn delete_ds_session_by_id(session_id: String) -> ApiResult<()> {
    let client = match client().await {
        Ok(value) => value,
        Err(value) => return value,
    };
    let request = tonic::Request::new(DeleteDsSessionRequest {
        id: session_id.clone(),
    });
    let response = client.clone().delete_ds_session(request).await;
    match response {
        Ok(_res) => {
            // res.into_inner();
            ApiResult::build(None)
        }
        Err(_) => ApiResult::new_with_err_status(None, AppStatus::LoginSessionFailed),
    }
}

pub async fn get_ds_session_by_user_id(user_id: i32) -> ApiResult<Vec<DsSession>> {
    let client = match client().await {
        Ok(value) => value,
        Err(value) => return value,
    };
    let request = tonic::Request::new(GetDsSessionUserIdRequest { user_id });
    let response = client.clone().get_ds_session_by_user_id(request).await;
    match response {
        Ok(res) => {
            let result = res.into_inner();
            ApiResult::build(Some(result.ds_sessions))
        }
        Err(_) => ApiResult::new_with_err_status(None, AppStatus::LoginSessionFailed),
    }
}

pub async fn create_ds_session(user: DsUser, extra: String) -> ApiResult<DsSession> {
    let client = match client().await {
        Ok(value) => value,
        Err(value) => return value,
    };
    info!("user: {:?}, extra: {}", user, extra);
    let user_id = user.id;
    let sessions = get_ds_session_by_user_id(user_id).await;
    match sessions.status {
        AppStatus::SUCCESS => {
            let sessions = sessions.data.unwrap();
            for (index, session) in sessions.iter().enumerate() {
                if index > 0 {
                    let id = session.id.clone();
                    let _ = delete_ds_session_by_id(id).await;
                }
            }
            let count = sessions.len();
            let new_session = if count == 0 {
                let ds_session = DsSession {
                    id: uuid::Uuid::new_v4().to_string(),
                    user_id,
                    ip: Some(extra),
                    last_login_time: Some("2024-09-11 12:12:12".to_string()),
                };
                let request = tonic::Request::new(CreateDsSessionRequest {
                    ds_session: Some(ds_session),
                });
                let response = client.clone().create_ds_session(request).await;
                match response {
                    Ok(res) => {
                        let session = res.into_inner();
                        return ApiResult::build(Some(session));
                    }
                    Err(e) => match e.code() {
                        tonic::Code::Unknown => {
                            let msg = e.message();
                            let err_info: DolphinErrorInfo = msg.parse().unwrap();
                            // eprintln!("err_info: {:?}", err_info);
                            // let r: AppStatus = err_info.into();
                            let res =
                                ApiResult::new_with_err_extra(None, err_info.into(), Some(vec![]));
                            error!("res: {:?}", res);
                            res
                        }
                        _ =>
                            ApiResult::new_with_err_status(None, AppStatus::InternalServerErrorArgs),
                    },
                }
            } else {
                let session = sessions.get(0).unwrap();
                let _ = update_ds_session(session.clone()).await;
                return ApiResult::build(Some(DsSession {
                    id: session.id.clone(),
                    user_id: session.user_id,
                    ip: session.ip.clone(),
                    last_login_time: session.last_login_time.clone(),
                }));
            };
            new_session
        }
        _ => ApiResult::new_with_err_status(None, sessions.status),
    }
}

pub async fn update_ds_session(ds_session: DsSession) -> ApiResult<()> {
    let client = match client().await {
        Ok(v) => v,
        Err(v) => return v,
    };
    let request = tonic::Request::new(UpdateDsSessionRequest {
        ds_session: Some(ds_session),
    });
    let response = client.clone().update_ds_session(request).await;
    match response {
        Ok(_res) => {
            // let session = res.into_inner();
            ApiResult::build(None)
        }
        Err(_) => ApiResult::new_with_err_status(None, AppStatus::LoginSessionFailed),
    }
}

async fn client<T>() -> Result<
    &'static proto::ds_session::ds_session_service_client::DsSessionServiceClient<
        tonic::transport::Channel,
    >,
    ApiResult<T>,
> {
    let client = match SESSION_SERVICE
        .get_or_init(|| async { _session_client().await })
        .await
    {
        Ok(client) => client,
        Err(_) => {
            return Err(ApiResult::<T>::new_with_err_status(
                None,
                AppStatus::InternalServerErrorArgs,
            ));
        }
    };
    Ok(client)
}
