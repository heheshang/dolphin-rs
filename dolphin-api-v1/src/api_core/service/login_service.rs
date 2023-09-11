use crate::api_core::{bean::response::ds_user_res::UserLoginInfoRes, security::get_authenticator};
use dolphin_common::core_status::app_status::AppStatus;


use tracing::info;

pub async fn login_service(
    user_name: String,
    user_password: String,
    extra: String,
) -> (Option<UserLoginInfoRes>, AppStatus) {
    info!(
        "user_name: {}, user_password: {}, extra: {}",
        user_name, user_password, extra
    );

    let authenticator = get_authenticator();
    let res = authenticator
        .authenticate(user_name.clone(), user_password.clone(), extra)
        .await;
    let res = match res.status {
        AppStatus::SUCCESS => match res.data {
            Some(data) => {
                let session_id = data.get("session_id").unwrap_or(&"".to_string()).clone();
                return (
                    Some(UserLoginInfoRes {
                        session_id: Some(session_id),
                    }),
                    AppStatus::SUCCESS,
                );
            }
            _ => (None, AppStatus::UserLoginFailure),
        },
        _ => (None, res.status),
    };

    // let mut pool = LocalPool::new();

    // let res = pool.run_until(async {
    //     let authenticator = get_authenticator();
    //     let res = authenticator
    //         .authenticate(user_name.clone(), user_password.clone(), extra)
    //         .await;
    //     match res.status {
    //         AppStatus::SUCCESS => match res.data {
    //             Some(data) => {
    //                 let session_id = data.get("session_id").unwrap_or(&"".to_string()).clone();
    //                 (
    //                     Some(UserLoginInfoRes {
    //                         session_id: Some(session_id),
    //                     }),
    //                     AppStatus::SUCCESS,
    //                 )
    //             }
    //             _ => (None, AppStatus::UserLoginFailure),
    //         },
    //         _ => (None, res.status),
    //     }
    // });

    // let res = block_on(async move {
    //     let authenticator = get_authenticator();
    //     let res = authenticator
    //         .authenticate(user_name.clone(), user_password.clone(), extra)
    //         .await;
    //     match res.status {
    //         AppStatus::SUCCESS => match res.data {
    //             Some(data) => {
    //                 let session_id = data.get("session_id").unwrap_or(&"".to_string()).clone();
    //                 (
    //                     Some(UserLoginInfoRes {
    //                         session_id: Some(session_id),
    //                     }),
    //                     AppStatus::SUCCESS,
    //                 )
    //             }
    //             _ => (None, AppStatus::UserLoginFailure),
    //         },
    //         _ => (None, res.status),
    //     }
    // });

    // let authenticator = get_authenticator();
    // let res = authenticator
    //     .authenticate(user_name.clone(), user_password.clone(), extra)
    //     .await;
    // match res.status {
    //     AppStatus::SUCCESS => match res.data {
    //         Some(data) => {
    //             let session_id = data.get("session_id").unwrap_or(&"".to_string()).clone();
    //             (
    //                 Some(UserLoginInfoRes {
    //                     session_id: Some(session_id),
    //                 }),
    //                 AppStatus::SUCCESS,
    //             )
    //         }
    //         _ => (None, AppStatus::UserLoginFailure),
    //     },
    //     _ => (None, res.status),
    // }
    // });

    res
}

#[cfg(test)]
mod tests {
    use futures::executor::block_on;
    use std::thread;
    use tokio::{task, time::Duration};
    #[tokio::test]
    async fn runtime_is_work() {
        let _res = block_on(async move {
            let s = task::spawn_blocking(move || {
                thread::sleep(Duration::from_secs(10));
                eprintln!("hello world");
                "12333".to_string()
            });
            s.await
        });
        eprintln!("hello world1")
    }
}
