use crate::repository::database::MongoClient;
use rocket::fs::NamedFile;
use rocket::serde::json::Json;
use rocket::serde::json::{json, Value};
use rocket::data::{Data, ToByteUnit};
use rocket::State;
use rocket::http::Status;
use crate::models::models::User;
use std::{fs, io};
use std::path::Path;

#[get("/")]
pub fn index() -> String{
    String::from("Hello World!!\n This is a Rust Server.")
}

#[get("/name")]
pub fn name() -> Value{
  let name: &str = "Aman Kansal";
  json!({
    "name": name,
    "id": 56
  })
}

#[post("/user", format="json", data="<input>")]
pub async fn user(db: &State<MongoClient>, input: Json<User>) -> Status{
  db.create_user(input.into_inner(), "Interface", "User")
  .await
  .expect("Can't create a user");
  Status::new(201)
}

#[get("/userdata/<id>")]
pub async fn userdata(db: &State<MongoClient>, id: u32) -> Value{
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
pub async fn post_image(id: u32, file: Data<'_>) -> Result<Status, io::Error>{
  let _img_file = fs::File::create_new(Path::new(format!("./temp/image{}.jpg", id).as_str())).unwrap();
  let img_path = format!("./temp/image{}.jpg", id);
  let path: &Path = Path::new(img_path.as_str());
  let data = file.open(2_usize.mebibytes());
  data.into_file(path).await?;
  Ok(Status::new(200))
} 

#[get("/getimg/<id>")]
pub async fn send_image(id: u32) -> Option<NamedFile>{
  let path_str = format!("./temp/image{}.jpg", id);
  let path = Path::new(path_str.as_str());
  if path.exists(){
    NamedFile::open(path).await.ok()
  }
  else{
    None
  }
}