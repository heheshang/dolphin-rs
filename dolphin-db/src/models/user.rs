use rbatis::{crud, executor::Executor, RBatis};
use rbdc::datetime::DateTime;
use rbs::to_value;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
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
crud!(User {}, "t_ds_user");

impl User {
    pub async fn query_all_general_user(rb: RBatis) -> Vec<User> {
        rb.query_decode("select * from t_ds_user where user_type=?", vec![
            to_value!(1),
        ])
        .await
        .unwrap()
    }

    pub async fn query_all_general_user1(
        rb: &mut dyn Executor,
    ) -> std::result::Result<Vec<User>, rbatis::rbdc::Error> {
        let v = rb
            .query("select * from t_ds_user where user_type=?", vec![
                to_value!(1),
            ])
            .await?;

        rbatis::decode::decode(v)
    }
}
