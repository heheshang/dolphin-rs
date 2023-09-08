use crate::{
    bean::{request::ds_user_req::UserInfoReq, response::ds_user_res::UserInfoRes},
    client::service::{user_client, USER_SERVICE},
};
use dolphin_common::{
    core_error::error::DolphinErrorInfo, core_results::results::ApiResult,
    core_status::app_status::AppStatus,
};
use proto::ds_user::{
    DsUserBean, GetDsUserBeanRequest, GetDsUserByIdRequest, QueryUserByNamePasswordRequest,
};
use serde::{Deserialize, Serialize};
use tracing::{error, info};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
}

impl UserInfoReq {
    pub async fn user_info(&self) -> ApiResult<UserInfoRes> {
        let client = match client().await {
            Ok(value) => value,
            Err(value) => return value,
        };

        let request = tonic::Request::new(GetDsUserBeanRequest {
            name: self.user_name.clone(),
        });
        let response = client.clone().get_ds_user_bean(request).await;
        match response {
            Ok(res) => {
                let user = res.into_inner();
                ApiResult::build(Some(user.into()))
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
                _ => ApiResult::new_with_err_status(None, AppStatus::InternalServerErrorArgs),
            },
        }
    }
}

pub async fn get_user_by_id(id: i32) -> ApiResult<DsUserBean> {
    let client = match client().await {
        Ok(value) => value,
        Err(value) => return value,
    };
    let request = tonic::Request::new(GetDsUserByIdRequest { id });
    client
        .clone()
        .get_ds_user_by_id(request)
        .await
        .map(|res| ApiResult::build(Some(res.into_inner().ds_user_bean)))
        .map_err(|e| {
            error!("get_user_by_id error: {:?}", e);
            ApiResult::new_with_err_status(None, AppStatus::InternalServerErrorArgs)
        })
        .unwrap_err()
}
pub async fn query_user_by_name_password(
    user_name: String,
    user_password: String,
    extra: String,
) -> ApiResult<DsUserBean> {
    info!(
        "query_user_by_name_password user_name: {:?} ,user_password: {:?},extra: {:?}",
        user_name, user_password, extra
    );
    let client = match client().await {
        Ok(value) => value,
        Err(value) => return value,
    };
    let digest = md5::compute(user_password);
    let request = tonic::Request::new(QueryUserByNamePasswordRequest {
        user_name,
        user_password: format!("{:x}", digest),
    });
    let res = client
        .clone()
        .query_user_by_name_password(request)
        .await
        .map(|res| ApiResult::build(Some(res.into_inner())))
        .map_err(|e| {
            error!("query_user_by_name_password error: {:?}", e);
            ApiResult::new_with_err_status(None, AppStatus::UserNamePasswdError)
        });

    info!("res: {:?}", res);
    match res {
        Ok(value) => value,
        Err(value) => value,
    }
}

async fn client<T>() -> Result<
    &'static proto::ds_user::ds_user_bean_service_client::DsUserBeanServiceClient<
        tonic::transport::Channel,
    >,
    ApiResult<T>,
> {
    let client = match USER_SERVICE
        .get_or_init(|| async { user_client().await })
        .await
    {
        Ok(client) => client,
        Err(_) => {
            return Err(ApiResult::new_with_err_status(
                None,
                AppStatus::InternalServerErrorArgs,
            ));
        }
    };
    Ok(client)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_query_user_by_name_password() {
        let res = query_user_by_name_password(
            "admin".to_string(),
            "dolphinscheduler123".to_string(),
            "c".to_string(),
        )
        .await;
        eprintln!("res: {:?}", res);
    }
}
