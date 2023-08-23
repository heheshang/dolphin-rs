use std::{env, net::SocketAddr, str::FromStr};

use anyhow::Result;
use axum::{error_handling::HandleErrorLayer, routing::post, BoxError, Router, Server};
use tower::ServiceBuilder;
use tower_governor::{errors::display_error, governor::GovernorConfigBuilder, GovernorLayer};
use tracing::info;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
pub fn main() {
    let res = start_server();
    if let Some(err) = res.err() {
        eprintln!("server error: {}", err);
    }
}

pub mod user;
pub use self::user::*;

#[tokio::main]
async fn start_server() -> Result<()> {
    env::set_var("RUST_LOG", "info");
    env::set_var("RUST_BACKTRACE", "1");
    let governor_conf = Box::new(
        GovernorConfigBuilder::default()
            .burst_size(10000)
            .per_second(60)
            .finish()
            .unwrap(),
    );

    let subscriber = tracing_subscriber::fmt::Subscriber::builder()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "axum_demo=debug,hyper=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with_writer(|| {
            // 创建一个用于将日志写入文件的闭包
            use std::{fs::OpenOptions, io::Write};
            let file = OpenOptions::new()
                .create(true)
                .append(true)
                .open("/home/ssk/workspace/rust-work/dolphin-rs/dolphin-api-v1/log_output.log")
                .expect("Failed to open log file");
            Box::new(file) as Box<dyn Write + Send>
        })
        .finish()
        .with(tracing_subscriber::fmt::layer().with_file(true));
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");
    dotenvy::dotenv().ok();
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");

    info!("server is running on http://{host}:{port}");
    let server_url = format!("{host}:{port}");

    let app = Router::new()
        .route("/authorize", post(authorize))
        .route("/get_user", post(get_user))
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|e: BoxError| async move {
                    tracing::error!("Unhandled error: {}", e);
                    display_error(e)
                }))
                .layer(GovernorLayer {
                    config: Box::leak(governor_conf),
                }),
        );

    // .route("/", get(list_posts).post(create_post))
    // .route("/:id", get(edit_post).post(update_post))
    // .route("/new", get(new_post))
    // .route("/delete/:id", post(delete_post))
    // .nest_service(
    //     "/static",
    //     get_service(ServeDir::new(concat!(
    //         env!("CARGO_MANIFEST_DIR"),
    //         "/static"
    //     )))
    //     .handle_error(|error: std::io::Error| async move {
    //         (
    //             StatusCode::INTERNAL_SERVER_ERROR,
    //             format!("Unhandled internal error: {error}"),
    //         )
    //     }),
    // )
    // .layer(CookieManagerLayer::new())
    // .with_state(state)

    let addr = SocketAddr::from_str(&server_url).unwrap();
    Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await?;

    Ok(())
}
