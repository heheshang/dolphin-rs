use dolphin_common::{core_results::results::ApiResult, core_status::app_status::AppStatus};

use crate::{
    bean::{request::ds_user_req::UserLoginInfoReq, response::ds_user_res::UserLoginInfoRes},
    client::client::{user_client, USER_SERVICE},
};

impl UserLoginInfoReq {
    pub async fn login(&self) -> ApiResult<UserLoginInfoRes> {
        let _client = match USER_SERVICE
            .get_or_init(|| async { user_client().await })
            .await
        {
            Ok(client) => client,
            Err(_) => {
                return ApiResult::new_with_err_status(None, AppStatus::InternalServerErrorArgs);
            }
        };

        // client
        todo!()
    }
}
