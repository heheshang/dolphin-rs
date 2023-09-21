use crate::api_core::service::{session_service, user_service};
use async_trait::async_trait;
use dolphin_common::{core_results::results::ApiResult, core_status::app_status::AppStatus};
use proto::ds_user::{DsUser as UserInfo, Flag};
use std::collections::HashMap;
use tracing::info;
pub enum AuthenticatorType {
    Password,
    Ldap,
}
impl AuthenticatorType {
    pub fn new(auth_type: String) -> Self {
        match auth_type.as_str() {
            "PASSWORD" => AuthenticatorType::Password,
            "LDAP" => AuthenticatorType::Ldap,
            _ => AuthenticatorType::Password,
        }
    }
}

#[async_trait]
pub trait Authenticator: Sync + Send {
    async fn login(&self, username: String, password: String, extra: String)
        -> ApiResult<UserInfo>;
    async fn authenticate(
        &self,
        username: String,
        password: String,
        extra: String,
    ) -> ApiResult<HashMap<String, String>> {
        let res = self.login(username, password, extra.clone()).await;

        match res.status {
            AppStatus::SUCCESS => match res.data {
                Some(data) => {
                    let session_res = session_service::create_ds_session(data, extra).await;
                    match session_res.data {
                        Some(session) => {
                            let mut map = HashMap::new();
                            map.insert("session_id".to_string(), session.id);
                            return ApiResult::build(Some(map));
                        }
                        _ => ApiResult::new_with_err_status(None, AppStatus::LoginSessionFailed),
                    }
                }
                _ => ApiResult::new_with_err_status(None, AppStatus::UserLoginFailure),
            },
            _ => ApiResult::new_with_err_extra(None, res.status, res.extra),
        }
    }
    async fn get_auth_user(&self, session_id: String) -> ApiResult<UserInfo> {
        let sesion_res = session_service::get_ds_session_by_id(session_id).await;
        match sesion_res.data {
            Some(s) => {
                let user_res = user_service::get_user_by_id(s.user_id).await;
                match user_res.data {
                    Some(u) => match Flag::from_i32(u.state.unwrap_or(0)) {
                        Some(Flag::Yes) => ApiResult::build(Some(u)),
                        Some(Flag::No) =>
                            ApiResult::new_with_err_status(None, AppStatus::UserDisabled),
                        None => ApiResult::new_with_err_status(None, AppStatus::LoginSessionFailed),
                    },
                    None => ApiResult::new_with_err_status(None, AppStatus::LoginSessionFailed),
                }
            }
            None => {
                return ApiResult::new_with_err_status(None, AppStatus::LoginSessionFailed);
            }
        }
    }
}

#[derive(Default)]
pub struct PasswordAuthenticator;
#[derive(Default)]
pub struct LdapAuthenticator;

#[async_trait]
impl Authenticator for PasswordAuthenticator {
    async fn login(
        &self,
        username: String,
        password: String,
        extra: String,
    ) -> ApiResult<UserInfo> {
        info!(
            "username:{},password:{},extra:{}",
            username, password, extra
        );
        user_service::query_user_by_name_password(username, password, extra.clone()).await
    }
}

#[async_trait]
impl Authenticator for LdapAuthenticator {
    async fn login(
        &self,
        username: String,
        password: String,
        extra: String,
    ) -> ApiResult<UserInfo> {
        info!(
            "username:{},password:{},extra:{}",
            username, password, extra
        );
        todo!()
    }
}
