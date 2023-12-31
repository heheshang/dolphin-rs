use axum::{extract::Path, http::Request, middleware::map_request, routing::get, Router};
use std::collections::HashMap;

pub async fn log_path_params<B>(
    Path(path_params): Path<HashMap<String, String>>,
    request: Request<B>,
) -> Request<B> {
    tracing::info!(?path_params);
    tracing::info!("uri: {:?}", request.uri());
    tracing::info!("method: {:?}", request.method());
    tracing::info!("headers: {:?}", request.headers());


    request
}
