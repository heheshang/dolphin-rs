use dolphin_common::{
    core_error::error::DolphinErrorInfo,
    core_results::results::ApiResult,
    core_status::app_status::AppStatus,
};
use proto::ds_user::GetDsUserBeanRequest;
use serde::{Deserialize, Serialize};

use tracing::error;

use crate::{
    bean::{request::ds_user_req::UserInfoReq, response::ds_user_res::UserInfoRes},
    client::client::{get_user_client, USER_SERVICE_CLIENT},
};


#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
}


impl UserInfoReq {
    pub async fn user_info(&self) -> ApiResult<UserInfoRes> {
        let client = match USER_SERVICE_CLIENT
            .get_or_init(|| async { get_user_client().await })
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
