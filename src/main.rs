use rocket::serde::json::{self, Json};
use rocket::serde::json::{json, Value};
use rocket::http::Method;
use rocket_cors::{AllowedOrigins, CorsOptions};

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> String{
    String::from("Hello World!!")
}

#[get("/name")]
fn name() -> Value{
  json!({
    "name": "Aman kansal",
    "id": 56
  })
}

#[launch]
fn rocket() -> _ {
    let cors = CorsOptions::default()
    .allowed_origins(AllowedOrigins::all())
    .allowed_methods(
        vec![Method::Get, Method::Post, Method::Patch]
            .into_iter()
            .map(From::from)
            .collect(),
    )
    .allow_credentials(true);

    rocket::build()
    .attach(cors.to_cors().unwrap())
    .mount("/", routes![index, name])
}
