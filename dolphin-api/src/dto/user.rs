use rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewUser {
    pub id: Option<i64>,
    pub user_name: String,
    pub user_password: String,
    pub user_type: i32,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub tenant_id: i64,
    pub create_time: DateTime,
    pub update_time: DateTime,
    pub queue: Option<String>,
    pub state: i32,
    pub time_zone: Option<String>,
}
rbatis::crud!(NewUser {}, "t_ds_user");
