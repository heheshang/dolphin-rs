use anyhow::Result;
use sea_orm::Database;
use std::net::SocketAddr;
use tonic::transport::Server;
pub mod service;

pub use service::*;

use crate::service::service::DolphinRpcServer;
#[tokio::main]
async fn main() -> Result<()> {
    let addr: SocketAddr = "0.0.0.0:50051".parse()?;

    // let database_url = env::var("postgres://superset:superset@tx:15432/dolphinscheduler").expect("DATABASE_URL must be set");

    // establish database connection
    let connection =
        Database::connect("postgres://superset:superset@tx:15432/dolphinscheduler").await?;
    println!("Hello, world!");
    // let hello_server = MyServer { connection };
    let grpc_server = DolphinRpcServer::new(connection);
    Server::builder()
        .add_service(grpc_server.clone().ds_user())
        .add_service(grpc_server.clone().ds_access_token())
        // .add_service(svc)
        .serve(addr)
        .await?;

    Ok(())
}
