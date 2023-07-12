use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    get,
    post,
    web,
    web::Json,
    App,
    Error,
    HttpResponse,
    HttpServer,
    Responder,
    Result,
};
use dolphin_db::User;
use futures_util::future::LocalBoxFuture;
use log::info;
use rbatis::RBatis;
use serde::{Deserialize, Serialize};
use std::{
    future::{ready, Ready},
    sync::Arc,
};
mod dto;
use dolphin_db::datasources::DbConnectionFactory;


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}


#[derive(Deserialize, Serialize)]
struct Message {
    id: u32,
    contents: String,
}

/// extract `Info` using serde
// async fn message(info: web::Json<Message>) -> Result<String> {
//     Ok(format!("Welcome {}!", info.contents))
// }
async fn message(info: Json<Message>) -> Result<impl Responder> {
    let obj = Message {
        id: info.id,
        contents: info.contents.clone(),
    };

    Ok(web::Json(obj))
}

// #[crud_table(table_name:"t_ds_user")]
// #[derive(Clone, Debug, Serialize, Deserialize)]
// #[pg_mapper(table = "t_ds_user")] // singular 'user' is a keyword..

async fn users(rb: web::Data<Arc<RBatis>>) -> impl Responder {
    let mut executor: rbatis::executor::RBatisConnExecutor = rb.acquire().await.unwrap();
    let user = User::select_all(&mut executor).await.unwrap();
    // let users = User::query_all_general_user1(&mut executor).await;
    info!("users: {:?}", user);
    HttpResponse::Ok().json(user)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let rb = DbConnectionFactory::builder("postgres://superset:superset@tx:15432/dolphinscheduler")
        .await
        .build()
        .await;


    let rb = Arc::new(rb);
    // env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log4rs::init_file("./log4rs.yaml", Default::default()).expect("init log4rs.yaml failed");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(rb.to_owned()))
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .route("/users", web::get().to(users))
            .service(web::resource("/message").route(web::post().to(message)))
            // .wrap(Logger::default())
        // .wrap(Logging)
        .wrap_fn(|req, srv| {
            info!("Hi from start. You requested: {}", req.path());
            info!("Hi from start. You requested: {:?}", req);
            let fut = srv.call(req);
            async {
                let res = fut.await?;
                info!("Hi from response: {:?}", res);
                Ok(res)
            }
        })
    })
    .bind(("127.0.0.1", 12345))?
    .run()
    .await
}

#[derive(Clone, Debug)]
pub struct Logging;

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for Logging
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Error = Error;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;
    type InitError = ();
    type Response = ServiceResponse<B>;
    type Transform = LoggingMiddleware<S>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(LoggingMiddleware { service }))
    }
}

pub struct LoggingMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for LoggingMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;
    type Response = ServiceResponse<B>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        info!("Hi from start. You requested: {}", req.path());
        info!("Hi from start. You requested: {:?}", req);

        let fut = self.service.call(req);


        Box::pin(async move {
            let res: ServiceResponse<B> = fut.await?;
            info!("Hi from response status: {:?}", res.status());

            for (header_name, header_value) in res.headers() {
                info!("Header: {}: {:?}", header_name, header_value); // 打印头部信息
            }

            Ok(res)
        })
    }
}
