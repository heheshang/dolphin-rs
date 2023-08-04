use actix_service::{Service, Transform};
use actix_web::{Error, HttpResponse};
use futures::future::{ok, Either, Ready};
use jsonwebtoken::{DecodingKey, Validation};

pub struct JwtMiddleware;

impl<S, B> Transform<S> for JwtMiddleware
where
    S: Service<
        Request = actix_web::HttpRequest,
        Response = actix_web::HttpResponse,
        Error = actix_web::Error,
    >,
    S::Future: 'static,
{
    type Error = actix_web::Error;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;
    type InitError = ();
    type Request = actix_web::HttpRequest;
    type Response = actix_web::HttpResponse;
    type Transform = JwtMiddlewareService<S>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(JwtMiddlewareService { service })
    }
}

pub struct JwtMiddlewareService<S> {
    service: S,
}

impl<S, B> Service for JwtMiddlewareService<S>
where
    S: Service<
        Request = actix_web::HttpRequest,
        Response = actix_web::HttpResponse,
        Error = actix_web::Error,
    >,
    S::Future: 'static,
{
    type Error = actix_web::Error;
    type Future = Either<S::Future, Ready<Result<Self::Response, Self::Error>>>;
    type Request = actix_web::HttpRequest;
    type Response = actix_web::HttpResponse;

    fn poll_ready(
        &mut self,
        ctx: &mut actix_service::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&mut self, req: actix_web::HttpRequest) -> Self::Future {
        let token = match req.headers().get("authorization") {
            Some(header_value) => {
                let header_str = header_value.to_str().unwrap();
                let parts: Vec<&str> = header_str.split(' ').collect();
                if parts.len() == 2 && parts[0] == "Bearer" {
                    parts[1]
                } else {
                    return Either::Right(ok(HttpResponse::Unauthorized().finish().into_body()));
                }
            }
            None => return Either::Right(ok(HttpResponse::Unauthorized().finish().into_body())),
        };

        let key = DecodingKey::from_secret("YOUR_SECRET_KEY".as_bytes());
        let validation = Validation::default();

        match jsonwebtoken::decode::<YourClaims>(&token, &key, &validation) {
            Ok(_) => Either::Left(self.service.call(req)),
            Err(_) => Either::Right(ok(HttpResponse::Unauthorized().finish().into_body())),
        }
    }
}

use std::future::{ready, Ready};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures_util::future::LocalBoxFuture;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct SayHi;

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for SayHi
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = SayHiMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(SayHiMiddleware { service }))
    }
}

pub struct SayHiMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for SayHiMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        println!("Hi from start. You requested: {}", req.path());

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;

            println!("Hi from response");
            Ok(res)
        })
    }
}
