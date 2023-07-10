// #![feature(decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_sync_db_pools;
use std::{borrow::Cow, net::Ipv4Addr};

use rocket::{
    launch,
    post,
    routes,
    serde::{json::Json, Deserialize, Serialize},
    Config,
};

mod diesel_postgres;

// use rocket_contrib::json::Json;
#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Message<'r> {
    id: i32,
    contents: Cow<'r, String>,
}

#[post("/message", format = "json", data = "<message>")]
fn message(message: Json<Message>) -> Json<Message> {
    format!("id: {}, contents: {}", message.id, message.contents);
    let pass = md5::compute(message.contents.as_bytes());
    Json(Message {
        id: message.id,
        contents: Cow::Owned(format!("{:x}", pass)),
    })
}

#[launch]
async fn rocket() -> _ {
    let config = Config {
        port: 12345,
        address: Ipv4Addr::new(0, 0, 0, 0).into(),
        temp_dir: "/tmp/config-example".into(),
        ..Config::debug_default()
    };

    rocket::custom(config)
        .mount("/", routes![message])
        .attach(diesel_postgres::stage())
}
