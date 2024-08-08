mod repository;
mod models;
use repository::database::MongoClient;
use rocket::fs::NamedFile;
use rocket::serde::json::Json;
use rocket::serde::json::{json, Value};
use rocket::data::{Data, ToByteUnit};
use rocket::State;
use rocket::http::Method;
use rocket_cors::{AllowedOrigins, CorsOptions};
use rocket::http::Status;
use models::models::User;
use std::{fs, io};
use std::path::Path;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> String{
    String::from("Hello World!!\n This is a Rust Server.")
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

#[post("/image/<id>", format="image/jpeg", data="<file>")]
async fn post_image(id: u32, file: Data<'_>) -> Result<Status, io::Error>{
  let _img_file = fs::File::create_new(Path::new(format!("./temp/image{}.jpg", id).as_str())).unwrap();
  let img_path = format!("./temp/image{}.jpg", id);
  let path: &Path = Path::new(img_path.as_str());
  let data = file.open(2_usize.mebibytes());
  data.into_file(path).await?;
  Ok(Status::new(200))
} 

#[get("/getimg/<id>")]
async fn send_image(id: u32) -> Option<NamedFile>{
  let path_str = format!("./temp/image{}.jpg", id);
  let path = Path::new(path_str.as_str());
  if path.exists(){
    NamedFile::open(path).await.ok()
  }
  else{
    None
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
    .mount("/", routes![post_image, send_image])
}
