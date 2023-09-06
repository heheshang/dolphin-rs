use crate::client::client::{session_client, SESSION_SERVICE};
use dolphin_common::{core_results::results::ApiResult, core_status::app_status::AppStatus};
use proto::ds_session::{DsSessionBean, GetDsSessionBeanByIdRequest};


pub async fn get_ds_session_by_id(session_id: String) -> ApiResult<DsSessionBean> {
    let client = match client().await {
        Ok(value) => value,
        Err(value) => return value,
    };

    let request = tonic::Request::new(GetDsSessionBeanByIdRequest {
        id: session_id.clone(),
    });
    let response = client.clone().get_ds_session_by_id(request).await;
    match response {
        Ok(res) => {
            let session = res.into_inner();
            ApiResult::new(Some(session))
        }
        Err(_) => ApiResult::new_with_err_status(None, AppStatus::LoginSessionFailed),
    }
}

async fn client() -> Result<
    &'static proto::ds_session::ds_session_bean_service_client::DsSessionBeanServiceClient<
        tonic::transport::Channel,
    >,
    ApiResult<DsSessionBean>,
> {
    let client = match SESSION_SERVICE
        .get_or_init(|| async { session_client().await })
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
