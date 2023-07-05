use std::net::Ipv4Addr;

use anyhow::Result;
use rocket::{get, serde::json::Json, Config};
use rocket_okapi::{
    openapi,
    openapi_get_routes,
    swagger_ui::{make_swagger_ui, SwaggerUIConfig},
    JsonSchema,
};

#[derive(serde::Serialize, JsonSchema)]
struct Response {
    reply: String,
}


#[openapi(tag = "Hello World")]
#[get("/hello/<number>")]
fn hello_world(number: i32) -> String {
    format!("Hello world number {}", number)
}

#[openapi]
#[get("/")]
fn index() -> Json<Response> {
    Json(Response {
        reply: "show me the docs!".to_string(),
    })
}

fn get_docs() -> SwaggerUIConfig {
    SwaggerUIConfig {
        url: "/openapi.json".to_string(),
        ..Default::default()
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config {
        port: 12345,
        address: Ipv4Addr::new(0, 0, 0, 0).into(),
        temp_dir: "/tmp/config-example".into(),
        ..Config::debug_default()
    };
    rocket::build()
        .configure(config)
        .mount("/", openapi_get_routes![index, hello_world])
        .mount("/swagger", make_swagger_ui(&get_docs()))
        .launch()
        .await?;
    Ok(())
}
