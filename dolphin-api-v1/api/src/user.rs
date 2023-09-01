use api_core::{
    bean::request::ds_user_req::{UserInfoReq, UserLoginInfoReq},
    service::user_service::User,
};
use axum::{
    async_trait,
    extract::{ConnectInfo, FromRequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json,
    RequestPartsExt,
};
use tracing::info;

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use std::{fmt::Display, net::SocketAddr};
static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("SECRET env var not set");
    Keys::new(secret.as_bytes())
});
// basic handler that responds with a static string
pub async fn root(ConnectInfo(_addr): ConnectInfo<SocketAddr>, _claims: Claims) -> String {
    "Hello, World!".to_string()
}

pub async fn create_user(
    _claims: Claims,
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        username: payload.username,
        password: "".to_string(),
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}
pub async fn get_user(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<UserInfoReq>,
) -> Response {
    payload.user_info().await.into_response()

    // info!("获取数据为{u:?}");
    // Ok(Json(u))
}

pub async fn login(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    Json(payload): Json<UserLoginInfoReq>,
) -> Response {
    payload.login().await.into_response()
    // info!("获取数据为{u:?}");
    // Ok(Json(u))
}


pub async fn authorize(
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
    info!("token: {}", token);
    Ok(Json(AuthBody::new(token)))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
pub struct CreateUser {
    pub username: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct AuthBody {
    pub access_token: String,
    pub token_type: String,
}

#[derive(Deserialize, Debug)]
pub struct AuthPayload {
    pub client_id: String,
    pub client_secret: String,
}
#[derive(Deserialize, Debug)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}
pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
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
pub struct Claims {
    pub sub: String,
    pub company: String,
    pub exp: u64,
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
where S: Send + Sync
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        info!("{:?}", parts);
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
