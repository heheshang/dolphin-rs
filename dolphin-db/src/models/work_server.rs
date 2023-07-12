use rbatis::crud;
use rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkerServer {
    pub id: i32,
    pub host: String,
    pub port: i32,
    pub zk_directory: String,
    pub res_info: String,
    pub create_time: DateTime,
    pub last_heartbeat_time: DateTime,
}
crud!(WorkerServer {}, "t_ds_worker_server");

impl WorkerServer {}
