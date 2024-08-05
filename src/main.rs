mod repository;
mod models;
#[allow(unused_imports)]
use rocket::serde::json::Json;
use rocket::serde::json::{json, Value};
use rocket::http::Method;
use rocket_cors::{AllowedOrigins, CorsOptions};
use models::models::User;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> String{
    String::from("Hello World!!")
}

#[get("/name")]
fn name() -> Value{
  let name: &str = "Aman Kansal";
  json!({
    "name": name,
    "id": 56
  })
}

#[post("/user", format="json", data="<input>")]
fn user(input: Json<User>) -> (){
  let _temp = input.into_inner();
  todo!()
}

#[get("/userdata")]
fn userdata() -> (){
  todo!()
}

#[launch]
fn rocket() -> _ {
    let cors = CorsOptions::default()
    .allowed_origins(AllowedOrigins::all())
    .allowed_methods(
        vec![Method::Get, Method::Post, Method::Patch, Method::Delete, Method::Options]
            .into_iter()
            .map(From::from)
            .collect(),
    )
    .allow_credentials(true);

    rocket::build()
    .attach(cors.to_cors().unwrap())
    .mount("/", routes![index])
    .mount("/", routes![name])
    .mount("/name", routes![user, userdata])
}
