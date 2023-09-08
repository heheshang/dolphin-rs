use crate::{bean::response::ds_user_res::UserLoginInfoRes, security::get_authenticator};
use dolphin_common::core_status::app_status::AppStatus;
use tokio::runtime;

pub fn login_service(
    user_name: String,
    user_password: String,
    extra: String,
) -> (Option<UserLoginInfoRes>, AppStatus) {
    let rt = runtime::Runtime::new().unwrap();

    let res = rt.block_on(async {
        let authenticator = get_authenticator();
        let result = authenticator
            .authenticate(user_name.clone(), user_password.clone(), extra)
            .await;
        result
    });
    match res.status {
        AppStatus::SUCCESS => match res.data {
            Some(data) => {
                let session_id = data.get("session_id").unwrap_or(&"".to_string()).clone();
                (
                    Some(UserLoginInfoRes {
                        session_id: Some(session_id),
                    }),
                    AppStatus::SUCCESS,
                )
            }
            _ => (None, AppStatus::UserLoginFailure),
        },
        _ => (None, res.status),
    }
}
