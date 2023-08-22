use std::env;

use format as f;
use proto::ds_user::{user_service_client::UserServiceClient, DsUser, GetUserRequest};
use serde::{Deserialize, Serialize};
use tokio::sync::OnceCell;
use tonic::transport::{Channel, Endpoint};

static USER_SERVICE_CLIENT: OnceCell<UserServiceClient<Channel>> = OnceCell::const_new();

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub username: String,
}

pub async fn get_client() -> UserServiceClient<Channel> {
    dotenvy::dotenv().ok();
    let host = env::var("DOLPHIN_DAO_CLIENT_HOST")
        .expect("HOST is not set in .env file")
        .clone();
    let port = env::var("DOLPHIN_DAO_CLIENT_PORT")
        .expect("PORT is not set in .env file")
        .clone();
    let addr_str = f!("http://{host}:{port}").clone();
    let addr = Endpoint::from_shared(addr_str);
    let client = UserServiceClient::connect(addr.unwrap()).await.unwrap();
    client
}

impl User {
    pub async fn find(&self) -> UserRes {
        let client = USER_SERVICE_CLIENT
            .get_or_init(|| {
                let client = get_client();
                client
            })
            .await
            .clone();
        let request = tonic::Request::new(GetUserRequest {
            name: self.username.clone(),
        });
        let response = client.clone().get_user(request).await.unwrap();
        let users = response.into_inner();
        users.into()
    }
}
impl From<DsUser> for UserRes {
    fn from(user: DsUser) -> Self {
        Self {
            id: user.id,
            user_name: user.user_name,
            user_password: user.user_password,
            user_type: user.user_type,
            email: user.email,
            phone: user.phone,
            tenant_id: user.tenant_id,
            create_time: user.create_time,
            update_time: user.update_time,
            queue: user.queue,
            state: user.state,
            time_zone: user.time_zone,
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UserRes {
    pub id: i32,
    pub user_name: Option<String>,
    pub user_password: Option<String>,
    pub user_type: Option<i32>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub tenant_id: Option<i32>,
    /// google.protobuf.Timestamp create_time=8
    pub create_time: Option<String>,
    /// optional google.protobuf.Timestamp update_time=9;
    pub update_time: Option<String>,

    pub queue: Option<String>,
    pub state: Option<i32>,
    pub time_zone: Option<String>,
}
