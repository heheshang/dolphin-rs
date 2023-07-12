// use std::net::Ipv4Addr;

// use anyhow::Result;

// use rocket::{
//     data::{self, FromData, ToByteUnit},
//     get,
//     http::{ContentType, Status},
//     post,
//     request::{self, FromRequest, Outcome, Request},
//     routes,
//     serde::json::Json,
//     Config,
//     Data,
// };
// use rocket_contrib::json::Json as Json2;
// use rocket_okapi::{
//     openapi,
//     openapi_get_routes,
//     swagger_ui::{make_swagger_ui, SwaggerUIConfig},
//     JsonSchema,
// };
// use serde_derive::{Deserialize, Serialize};

// // #[derive(OpenApiFromRequest)]
// // pub struct MyStructName;
// #[derive(serde::Serialize, JsonSchema)]
// struct Response {
//     reply: String,
// }
// // #[get("/login")]
// // fn login() -> Template { /* .. */
// // }
// #[derive(Debug)]
// // #[serde(crate = "rocket::serde")]
// // #[derive(FromForm)]
// struct User {
//     id: String,
//     is_admin: bool,
// }


// struct Database;
// impl Database {
//     fn get_user(&self, id: String) -> Result<User, ()> {
//         Ok(User {
//             id,
//             is_admin: false,
//         })
//     }
// }
// #[rocket::async_trait]
// impl<'r> FromRequest<'r> for Database {
//     type Error = ();

//     async fn from_request(request: &'r Request<'_>) -> Outcome<Database, ()> {
//         Outcome::Success(Database)
//     }
// }

// #[derive(Debug, Serialize, Deserialize)]
// struct Message {
//     id: String,
//     content: String,
// }

// #[post("/", data = "<message>")]
// fn meassage(message: Json2<Message>) -> String {
//     format!("id: {}, contents: {}", message.id, message.content)
// }


// // #[get("/admin")]
// // fn admin_panel(admin: AdminUser) -> &'static str {
// // "Hello, administrator. This is the admin panel!"
// // }
// #[derive(Debug)]
// enum Error {
//     TooLarge,
//     NoColon,
//     InvalidAge,
//     Io(std::io::Error),
// }
// ///

// #[rocket::async_trait]
// impl<'r> rocket::data::FromData<'r> for User {
//     type Error = Error;

//     async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> data::Outcome<'r, Self> {
//         use rocket::outcome::Outcome::*;
//         use Error::*;
//         let limit = req.limits().get("user").unwrap_or(256.bytes());
//         let string = match data.open(limit).into_string().await {
//             Ok(string) if string.is_complete() => string.into_inner(),
//             Ok(_) => return Failure((Status::PayloadTooLarge, TooLarge)),
//             Err(e) => return Failure((Status::InternalServerError, Io(e))),
//         };
//         let string = request::local_cache!(req, string);
//         println!("req: {:?}", req);
//         let db = req.rocket().state::<Database>().unwrap();
//         let id = req.headers().get_one("id").unwrap().to_string();
//         let user = db.get_user(id).unwrap();
//         Success(user)
//     }
// }

// // #[openapi(tag = "Admin Panel")]
// #[post("/admin", format = "json", data = "<user>")]
// fn admin_panel_user(user: User) -> &'static str {
//     eprintln!("user: {:?}", user);
//     eprintln!("user: {:?}", user.id);
//     eprintln!("user: {:?}", user.is_admin);
//     "Sorry, you must be an administrator to access this page."
// }
// // #[get("/admin", rank = 3)]
// // fn admin_panel_redirect() -> Redirect {
// //     Redirect::to(uri!(login))
// // }

// #[openapi(tag = "Hello World")]
// #[get("/hello/<number>")]
// fn hello_world(number: i32) -> String {
//     format!("Hello world number {}", number)
// }

// #[openapi]
// #[get("/")]
// fn index() -> Json<Response> {
//     Json(Response {
//         reply: "show me the docs!".to_string(),
//     })
// }

// fn get_docs() -> SwaggerUIConfig {
//     SwaggerUIConfig {
//         url: "/openapi.json".to_string(),
//         ..Default::default()
//     }
// }

// #[tokio::main]
// async fn main() -> Result<()> {
//     let config = Config {
//         port: 12345,
//         address: Ipv4Addr::new(0, 0, 0, 0).into(),
//         temp_dir: "/tmp/config-example".into(),
//         ..Config::debug_default()
//     };
//     rocket::build()
//         .configure(config)
//         .mount("/", openapi_get_routes![index, hello_world,])
//         .mount("/", routes![admin_panel_user])
//         .mount("/meassage", routes![meassage])
//         .mount("/swagger", make_swagger_ui(&get_docs()))
//         .launch()
//         .await?;
//     Ok(())
// }

// use anyhow::Result;
// #[tokio::main]
// async fn main() -> Result<()> {
//     Ok(())
// }

fn main() {
    println!("Hello, world!");
}
