#[allow(unused_imports)]
use rocket::serde::json::Json;
use rocket::serde::json::{json, Value};
use rocket::http::Method;
use rocket_cors::{AllowedOrigins, CorsOptions};
use serde::{Deserialize, Serialize};


#[macro_use] extern crate rocket;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
struct User{
  id: u32,
  name: String,
  balance: u32,
  above_18: bool
}

static mut DATA: Vec<User> = Vec::new();

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
  let temp = input.into_inner();
  unsafe{DATA.push(temp);}
}

#[get("/userdata")]
fn userdata() -> Value{
  unsafe{
    json!({"data": DATA})
  }
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
