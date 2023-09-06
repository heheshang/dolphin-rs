use std::collections::HashMap;

use crate::client::client::{session_client, user_client, SESSION_SERVICE, USER_SERVICE};

use crate::service::user_service;
use async_trait::async_trait;
use dolphin_common::{core_results::results::ApiResult, core_status::app_status::AppStatus};
use proto::{
    ds_session::GetDsSessionBeanByIdRequest,
    ds_user::{DsUserBean as UserInfo, Flag, GetDsUserByIdRequest},
};
use tracing::info;
pub enum AuthenticatorType {
    Password(String),
    Ldap(String),
}

#[async_trait]
pub trait Authenticator {
    async fn login(&self, username: String, password: String, extra: String)
        -> ApiResult<UserInfo>;
    async fn authenticate(
        &self,
        username: String,
        password: String,
        extra: String,
    ) -> ApiResult<HashMap<String, String>> {
        let res = self.login(username, password, extra).await;
        match res.status {
            AppStatus::SUCCESS => {
                let _user = res.data.unwrap();
                // let mut map = HashMap::with_capacity(1);
                // map.insert("session_id".to_string(), user.session_id);
                ApiResult::new(None)
            }
            _ => ApiResult::new_with_err_extra(None, res.status, res.extra),
        }

        // todo!()
    }
    async fn get_auth_user(session_id: String) -> ApiResult<UserInfo> {
        let client = match SESSION_SERVICE
            .get_or_init(|| async { session_client().await })
            .await
        {
            Ok(client) => client,
            Err(_) => {
                return ApiResult::new_with_err_status(None, AppStatus::InternalServerErrorArgs);
            }
        };

        let request = tonic::Request::new(GetDsSessionBeanByIdRequest {
            id: session_id.clone(),
        });
        let response = client.clone().get_ds_session_by_id(request).await;
        match response {
            Ok(res) => {
                let session = res.into_inner();
                let user_id = session.user_id;
                let user_client = match USER_SERVICE
                    .get_or_init(|| async { user_client().await })
                    .await
                {
                    Ok(client) => client,
                    Err(_) => {
                        return ApiResult::new_with_err_status(None, AppStatus::LoginSessionFailed);
                    }
                };
                let request = tonic::Request::new(GetDsUserByIdRequest {
                    id: user_id.clone(),
                });
                let response = user_client.clone().get_ds_user_by_id(request).await;
                match response {
                    Ok(res) => {
                        let user = res.into_inner().ds_user_bean;
                        match user {
                            Some(u) => match u.state {
                                Some(state) => match Flag::from_i32(state) {
                                    Some(Flag::Yes) => ApiResult::new(Some(u)),
                                    Some(Flag::No) => ApiResult::new_with_err_status(
                                        None,
                                        AppStatus::UserDisabled,
                                    ),
                                    None => ApiResult::new_with_err_status(
                                        None,
                                        AppStatus::LoginSessionFailed,
                                    ),
                                },
                                None => ApiResult::new_with_err_status(
                                    None,
                                    AppStatus::LoginSessionFailed,
                                ),
                            },
                            None =>
                                ApiResult::new_with_err_status(None, AppStatus::LoginSessionFailed),
                        }
                    }
                    Err(_) => ApiResult::new_with_err_status(None, AppStatus::LoginSessionFailed),
                }
            }
            Err(_) => ApiResult::new_with_err_status(None, AppStatus::LoginSessionFailed),
        }
    }
}


pub struct PasswordAuthenticator;

pub struct LdapAuthenticator;


impl PasswordAuthenticator {
    pub fn new() -> Self {
        PasswordAuthenticator {}
    }
}
impl LdapAuthenticator {
    pub fn new() -> Self {
        LdapAuthenticator {}
    }
}
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
        user_service::query_user_by_name_password(username, password, extra).await
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


trait Product {}

trait Factory {
    fn new() -> Box<dyn Product>;
}

struct ConcreteFactory;


impl Factory for ConcreteFactory {
    fn new() -> Box<dyn Product> {
        Box::new(ConcreteProduct::new())
    }
}

struct ConcreteProduct;
impl ConcreteProduct {
    fn new() -> Self {
        ConcreteProduct {}
    }
}
impl Product for ConcreteProduct {}
