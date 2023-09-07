use crate::{
    bean::response::ds_user_res::UserLoginInfoRes,
    security::get_authenticator,
};
use dolphin_common::{core_results::results::ApiResult, core_status::app_status::AppStatus};

pub async fn login_service(
    user_name: String,
    user_password: String,
    extra: String,
) -> ApiResult<UserLoginInfoRes> {
    let authenticator = get_authenticator();
    let res = authenticator
        .authenticate(user_name.clone(), user_password.clone(), extra)
        .await;
    match res.status {
        AppStatus::SUCCESS => match res.data {
            Some(data) => {
                let session_id = data.get("session_id").unwrap_or(&"".to_string()).clone();
                ApiResult::build(Some(UserLoginInfoRes {
                    session_id: Some(session_id),
                }))
            }
            _ => ApiResult::new_with_err_status(None, AppStatus::UserLoginFailure),
        },
        _ => ApiResult::new_with_err_extra(None, res.status, res.extra),
    }
}
