mod repository;
mod models;
use repository::database::MongoClient;
#[allow(unused_imports)]
use rocket::serde::json::Json;
use rocket::serde::json::{json, Value};
use rocket::State;
use rocket::http::Method;
use rocket_cors::{AllowedOrigins, CorsOptions};
use rocket::http::Status;
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
async fn user(db: &State<MongoClient>, input: Json<User>) -> Status{
  db.create_user(input.into_inner(), "Interface", "User")
  .await
  .expect("Can't create a user");
  Status::new(201)
}

#[get("/userdata/<id>")]
async fn userdata(db: &State<MongoClient>, id: u32) -> Value{
  let user =  db.find_user("Interface", "User", id)
  .await
  .expect("User not Found");
  if user.is_none(){
    json!({
      "Error": "user not Found"
  })}
  else{
    json!(user)
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

    let db = MongoClient::init();

    rocket::build()
    .manage(db)
    .attach(cors.to_cors().unwrap())
    .mount("/", routes![index])
    .mount("/", routes![name])
    .mount("/name", routes![user, userdata])
}
