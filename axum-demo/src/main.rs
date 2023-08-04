use axum::{
    async_trait,
    body::Bytes,
    error_handling::HandleErrorLayer,
    extract::{ConnectInfo, FromRequestParts, MatchedPath, TypedHeader},
    headers::{authorization::Bearer, Authorization},
    http::{request::Parts, HeaderMap, Request, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    BoxError, Json, RequestPartsExt, Router,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use multiplex_service::MultiplexService;
use once_cell::sync::Lazy;
use proto::{greeter_server::Greeter, HelloReply, HelloRequest};
use serde::{Deserialize, Serialize};
use tonic::{Response as TonicResponse, Status};
use tower::ServiceBuilder;
use tower_governor::{errors::display_error, governor::GovernorConfigBuilder, GovernorLayer};

use std::{fmt::Display, net::SocketAddr, time::Duration};
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing::{info_span, Span};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::proto::greeter_server::GreeterServer;
mod multiplex_service;
pub(crate) mod proto {
    // tonic::include_proto!("helloworld");
    include!(concat!("./pb", "/helloworld.rs"));
    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("helloworld_descriptor");
}
// this is a macro that expands into an async function that returns a `Router`
//
//
static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("SECRET env var not set");
    Keys::new(secret.as_bytes())
});

#[derive(Default)]
struct GrpcServiceImpl {}

#[tonic::async_trait]
impl Greeter for GrpcServiceImpl {
    async fn say_hello(
        &self,
        request: tonic::Request<HelloRequest>,
    ) -> Result<TonicResponse<HelloReply>, Status> {
        tracing::info!("Got a request from {:?}", request.remote_addr());

        let reply = HelloReply {
            message: format!("Hqqqqqqqqqqqqqqqqqqqqqqello {}!", request.into_inner().name),
        };

        Ok(TonicResponse::new(reply))
    }
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "axum_demo=debug,hyper=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let governor_conf = Box::new(
        GovernorConfigBuilder::default()
            .burst_size(1)
            .per_second(1)
            .finish()
            .unwrap(),
    );
    // build our application with a route
    let rest = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user))
    //     // `POST /authorize` goes to `authorize`
        .route("/authorize",post(authorize))
          .route_layer(TraceLayer::new_for_http()
           .make_span_with(|request:&Request<_>|{

               let match_path = request.extensions().get::<MatchedPath>().map(MatchedPath::as_str);

               info_span!("http_request",method = ?request.method(),match_path,some_other_field = tracing::field::Empty)
           })
           .on_request(|_request:&Request<_>,_span:&Span|{
               // do something with the request
           })
           .on_response(|_response:&Response<_>,_latency:Duration,_span:&Span|{
               // do something with the response
           })
           .on_body_chunk(|_chunk:&Bytes,_latency:Duration,_span:&Span|{})
           .on_eos(|_trailers:Option<&HeaderMap>,_stream_duration:Duration,_span:&Span|{
               // do something with the trailers
           },)
           .on_failure(|_errors:ServerErrorsFailureClass,_latency:Duration,_span:&Span|{
               // do something with the errors
           }),
      )
      .layer(ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|e:BoxError| async move{
            tracing::error!("Unhandled error: {}",e);
            display_error(e)
        }))
        .layer(GovernorLayer{config:Box::leak(governor_conf)})
    );
    // build the grpc service
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();
    let grpc = tonic::transport::Server::builder()
        .add_service(reflection_service)
        .add_service(GreeterServer::new(GrpcServiceImpl::default()))
        .into_service();

    // combine them into one service
    let service = MultiplexService::new(
        // rest.into_make_service_with_connect_info::<SocketAddr>(),
        rest, grpc,
    );

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        // .serve(rest.into_make_service_with_connect_info::<SocketAddr>())
        .serve(tower::make::Shared::new(service))
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root(ConnectInfo(_addr): ConnectInfo<SocketAddr>, _claims: Claims) -> String {
    "Hello, World!".to_string()
}

async fn create_user(
    _claims: Claims,
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

async fn authorize(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<AuthPayload>,
) -> Result<Json<AuthBody>, AuthError> {
    // insert your application logic here

    //
    if payload.client_id.is_empty() || payload.client_secret.is_empty() {
        return Err(AuthError::MissingCredentials);
    }
    // if payload.client_id != "foo" || payload.client_secret != "bar" {
    //     return Err(AuthError::WrongCredentials);
    // }
    let claims = Claims {
        sub: payload.client_id,
        company: payload.client_secret,
        exp: 20_000_000_000,
    };
    let token = encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| AuthError::TokenCreation)?;
    Ok(Json(AuthBody::new(token)))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}

#[derive(Deserialize, Debug, Serialize)]
struct AuthBody {
    access_token: String,
    token_type: String,
}

#[derive(Deserialize, Debug)]
struct AuthPayload {
    client_id: String,
    client_secret: String,
}
#[derive(Deserialize, Debug)]
enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}
struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}
impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}
#[derive(Deserialize, Debug, Serialize, Clone)]
struct Claims {
    sub: String,
    company: String,
    exp: u64,
}

impl Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "sub: {}, company: {}, exp: {}",
            self.sub, self.company, self.exp
        )
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        tracing::info!("{:?}", parts);
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::MissingCredentials)?;

        let token_data = decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
            .map_err(|_| AuthError::InvalidToken)?;
        Ok(token_data.claims)
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::UNAUTHORIZED, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token"),
        };
        let body = Json(serde_json::json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}
impl AuthBody {
    fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::Error;

    use super::*;
    #[derive(Deserialize, Debug, Serialize)]
    pub struct MyClaims {
        sub: String,
        company: String,
        exp: u64,
    }

    #[tokio::test]
    async fn test_jwt_token_is_work() -> Result<(), Error> {
        let claims = MyClaims {
            sub: "foo".to_owned(),
            company: "bar".to_owned(),
            exp: 20_0000_0000,
            // exp:20_0000,
        };
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret("secret".as_ref()),
        )
        .unwrap();
        eprintln!("{}", token);
        let token = decode::<MyClaims>(
            &token,
            &DecodingKey::from_secret("secret".as_ref()),
            &Validation::default(),
        )
        .unwrap();
        eprintln!("{:?}", token.claims);

        // let header = decode_header(&token);
        // eprintln!("{:?}", header.unwrap());

        Ok(())
    }
}
