use anyhow::Result;
use config::{ConfigError, Config, Environment, File};
use sea_orm::Database as SeaDatabase;
use std::net::SocketAddr;
use tonic::transport::Server as TonicServer;
use serde::Deserialize;
use dolphin_config::get_dao_config_path;
use std::env;
pub mod service;


pub use service::*;

use crate::service::service::DolphinRpcServer;
#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Database {
    url: String,
}
#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Server {
    host:String,
    port:u16,
}
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    debug: bool,
    database: Database,
    server: Server,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
        let config_path = get_dao_config_path();
        let s = Config::builder()
            // Start off by merging in the "default" configuration file
            .add_source(File::with_name(config_path.join("default").to_str().unwrap()))
            // Add in the current environment file
            // Default to 'development' env
            // Note that this file is _optional_
            .add_source(
                File::with_name(config_path.join( run_mode).to_str().unwrap())
                    .required(false),
            )
            // Add in a local configuration file
            // This file shouldn't be checked in to git
            .add_source(File::with_name(config_path.join("local").to_str().unwrap()).required(false))
            // Add in settings from the environment (with a prefix of APP)
            // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
            .add_source(Environment::with_prefix("app"))
            // You may also programmatically change settings
            //.set_override("database.url", "postgres://")?
            .build()?;

        // Now that we're done, let's access our configuration
        println!("debug: {:?}", s.get_bool("debug"));
        //println!("database: {:?}", s.get::<String>("database.url"));

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_deserialize()
    }
}

#[tokio::main]
async fn main() -> Result<()> {

    //let _addr: SocketAddr = "0.0.0.0:50051".parse()?;

    // let database_url = env::var("postgres://superset:superset@tx:15432/dolphinscheduler").expect("DATABASE_URL must be set");

    // establish database connection
    let settings = Settings::new()?;
    let url = settings.database.url;
    let host = settings.server.host;
    let port = settings.server.port;
    let addr: SocketAddr = format!("{}:{}", host, port).parse()?;

    let connection =
        SeaDatabase::connect(url).await?;
    println!("Hello, world!");
    // let hello_server = MyServer { connection };
    let grpc_server = DolphinRpcServer::new(connection);
    // let ds_access_token = grpc_server.ds_access_token();
    TonicServer::builder()
        .add_optional_service(Some(grpc_server.clone().ds_access_token()))
        .add_optional_service(Some(grpc_server.clone().ds_alert()))
        .add_optional_service(Some(grpc_server.clone().ds_alert_plugin_instance()))
        .add_optional_service(Some(grpc_server.clone().ds_alert_send_status()))
        .add_optional_service(Some(grpc_server.clone().ds_alertgroup()))
        .add_optional_service(Some(grpc_server.clone().ds_audit_log()))
        .add_optional_service(Some(grpc_server.clone().ds_command()))
        .add_optional_service(Some(grpc_server.clone().ds_datasource()))
        .add_optional_service(Some(grpc_server.clone().ds_dq_comparison_type()))
        .add_optional_service(Some(grpc_server.clone().ds_dq_execute_result()))
        .add_optional_service(Some(grpc_server.clone().ds_dq_rule()))
        .add_optional_service(Some(grpc_server.clone().ds_dq_rule_execute_sql()))
        .add_optional_service(Some(grpc_server.clone().ds_dq_rule_input_entry()))
        .add_optional_service(Some(grpc_server.clone().ds_dq_task_statistics_value()))
        .add_optional_service(Some(grpc_server.clone().ds_environment()))
        .add_optional_service(Some(
            grpc_server.clone().ds_environment_worker_group_relation(),
        ))
        .add_optional_service(Some(grpc_server.clone().ds_error_command()))
        .add_optional_service(Some(grpc_server.clone().ds_k8s()))
        .add_optional_service(Some(grpc_server.clone().ds_k8s_namespace()))
        .add_optional_service(Some(grpc_server.clone().ds_plugin_define()))
        .add_optional_service(Some(grpc_server.clone().ds_process_definition()))
        .add_optional_service(Some(grpc_server.clone().ds_process_definition_log()))
        .add_optional_service(Some(grpc_server.clone().ds_process_instance()))
        .add_optional_service(Some(grpc_server.clone().ds_process_task_relation()))
        .add_optional_service(Some(grpc_server.clone().ds_process_task_relation_log()))
        .add_optional_service(Some(grpc_server.clone().ds_project()))
        .add_optional_service(Some(grpc_server.clone().ds_queue()))
        .add_optional_service(Some(grpc_server.clone().ds_relation_datasource_user()))
        .add_optional_service(Some(grpc_server.clone().ds_relation_namespace_user()))
        .add_optional_service(Some(grpc_server.clone().ds_relation_process_instance()))
        .add_optional_service(Some(grpc_server.clone().ds_relation_project_user()))
        .add_optional_service(Some(grpc_server.clone().ds_relation_resources_user()))
        .add_optional_service(Some(grpc_server.clone().ds_relation_rule_execute_sql()))
        .add_optional_service(Some(grpc_server.clone().ds_relation_rule_input_entry()))
        .add_optional_service(Some(grpc_server.clone().ds_relation_udfs_user()))
        .add_optional_service(Some(grpc_server.clone().ds_resources()))
        .add_optional_service(Some(grpc_server.clone().ds_schedules()))
        .add_optional_service(Some(grpc_server.clone().ds_session()))
        .add_optional_service(Some(grpc_server.clone().ds_task_definition()))
        .add_optional_service(Some(grpc_server.clone().ds_task_definition_log()))
        .add_optional_service(Some(grpc_server.clone().ds_task_group()))
        .add_optional_service(Some(grpc_server.clone().ds_task_group_queue()))
        .add_optional_service(Some(grpc_server.clone().ds_task_instance()))
        .add_optional_service(Some(grpc_server.clone().ds_tenant()))
        .add_optional_service(Some(grpc_server.clone().ds_udfs()))
        .add_optional_service(Some(grpc_server.clone().ds_user()))
        .add_optional_service(Some(grpc_server.clone().ds_version()))
        .add_optional_service(Some(grpc_server.clone().ds_worker_group()))
        .add_optional_service(Some(grpc_server.clone().qrtz_blob_triggers()))
        .add_optional_service(Some(grpc_server.clone().qrtz_calendars()))
        .add_optional_service(Some(grpc_server.clone().qrtz_cron_triggers()))
        .add_optional_service(Some(grpc_server.clone().qrtz_fired_triggers()))
        .add_optional_service(Some(grpc_server.clone().qrtz_job_details()))
        .add_optional_service(Some(grpc_server.clone().qrtz_locks()))
        .add_optional_service(Some(grpc_server.clone().qrtz_paused_trigger_grps()))
        .add_optional_service(Some(grpc_server.clone().qrtz_scheduler_state()))
        .add_optional_service(Some(grpc_server.clone().qrtz_simple_triggers()))
        .add_optional_service(Some(grpc_server.clone().qrtz_simprop_triggers()))
        .add_optional_service(Some(grpc_server.clone().qrtz_triggers()))
        .serve(addr)
        .await?;

    Ok(())
}
