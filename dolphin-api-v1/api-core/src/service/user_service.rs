use std::env;

use anyhow::Result;
use dolphin_common::{
    core_error::error::DolphinErrorInfo,
    core_results::results::ApiResult,
    core_status::app_status::AppStatus,
};
use format as f;
use proto::ds_user::{ds_user_bean_service_client::DsUserBeanServiceClient, GetDsUserBeanRequest};
use serde::{Deserialize, Serialize};

use tokio::sync::OnceCell;
use tonic::transport::{Channel, Endpoint};
use tracing::error;

use crate::bean::{
    request::ds_user_req::{UserInfoReq, UserLoginInfoReq},
    response::ds_user_res::{UserInfoRes, UserLoginInfoRes},
};

static USER_SERVICE_CLIENT: OnceCell<Result<DsUserBeanServiceClient<Channel>>> =
    OnceCell::const_new();

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
}

pub async fn get_client() -> Result<DsUserBeanServiceClient<Channel>> {
    dotenvy::dotenv().ok();
    let host = env::var("DOLPHIN_DAO_CLIENT_HOST")
        .expect("HOST is not set in .env file")
        .clone();
    let port = env::var("DOLPHIN_DAO_CLIENT_PORT")
        .expect("PORT is not set in .env file")
        .clone();
    let addr_str = f!("http://{host}:{port}").clone();
    let addr = Endpoint::from_shared(addr_str);
    match DsUserBeanServiceClient::connect(addr.unwrap()).await {
        Ok(client) => Ok(client),
        Err(e) => Err(anyhow::Error::new(e)),
    }
}

impl UserInfoReq {
    pub async fn user_info(&self) -> ApiResult<UserInfoRes> {
        let client = match USER_SERVICE_CLIENT
            .get_or_init(|| async { get_client().await })
            .await
        {
            Ok(client) => client,
            Err(_) => {
                return ApiResult::new_with_err_status(None, AppStatus::InternalServerErrorArgs);
            }
        };


        let request = tonic::Request::new(GetDsUserBeanRequest {
            name: self.user_name.clone(),
        });
        let response = client.clone().get_ds_user_bean(request).await;
        match response {
            Ok(res) => {
                let user = res.into_inner();
                ApiResult::new(Some(user.into()))
            }
            Err(e) => match e.code() {
                tonic::Code::Unknown => {
                    let msg = e.message();
                    let err_info: DolphinErrorInfo = msg.parse().unwrap();
                    // eprintln!("err_info: {:?}", err_info);
                    // let r: AppStatus = err_info.into();

                    let res = ApiResult::new_with_err_extra(
                        None,
                        err_info.into(),
                        Some(vec![self.user_name.clone()]),
                    );
                    error!("res: {:?}", res);
                    res
                }
                _ => {
                    return ApiResult::new_with_err_status(
                        None,
                        AppStatus::InternalServerErrorArgs,
                    );
                }
            },
        }
    }
}

impl UserLoginInfoReq {
    pub async fn login(&self) -> ApiResult<UserLoginInfoRes> {
        todo!()
    }
}
