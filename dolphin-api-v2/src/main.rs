mod cypt;
mod log;
mod model;
mod utils;
mod web;

mod tests;
use axum::{routing::get, Router, middleware};
use std::{env, net::SocketAddr};
use tracing::{info, Level};
use web::routes_user;

use crate::web::mw::mw_res_map::mw_response_map;
async fn hello() -> &'static str {
    info!("hello world");
    "hello world"
}


#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "info");
    env::set_var("RUST_BACKTRACE", "1");
    tracing_subscriber::fmt()
        .without_time()
        .with_max_level(Level::INFO)
        .with_target(true)
        .with_thread_names(true)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    info!("log init success!");
    let addr: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 54321));
    info!("{:<12}->{}", "listen", addr);
    let route_all = Router::new()
        .merge(routes_user::routes())
        .route("/api", get(hello))
        .layer(middleware::map_response(mw_response_map));
    axum::Server::bind(&addr)
        .serve(route_all.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}
