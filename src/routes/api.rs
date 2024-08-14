use crate::repository::database::MongoClient;
use crate::models::models::User;
use rocket::fs::NamedFile;
use rocket::serde::json::Json;
use rocket::serde::json::{json, Value};
use rocket::data::{Data, ToByteUnit};
use rocket::State;
use rocket::http::Status;
use std::{fs, io};
use std::path::Path;

#[get("/")]
pub fn index() -> String{
    String::from("Hello World!!\nThis is a Rust Server.")
}

#[post("/user", format="application/json", data="<input>")]
pub async fn user_sign_in(db: &State<MongoClient>, input: Json<User>) -> Value{
  if db.user_exists("Interface", "User", input.clone().into_inner().email).await {
    if db.credentials_ok("Interface", "User", input.clone().into_inner().name, input.clone().into_inner().email, input.clone().into_inner().password).await{
      let id = db
      .find_user_id("Interface", "User", input.into_inner().email)
      .await
      .ok()
      .unwrap()
      .unwrap()
      .id;
      json!({"id": id})
    }
    else{
      json!({
        "id": "Wrong Credentials"
      })
    }
  }
  else{
    db
    .create_user(input.clone().into_inner(), "Interface", "User")
    .await
    .ok();
    let id = db
    .find_user_id("Interface", "User", input.into_inner().email)
    .await
    .ok()
    .unwrap()
    .unwrap()
    .id;
    json!({"id": id})
  }
}

#[get("/userdata/<id>")]
pub async fn get_user(db: &State<MongoClient>, id: String) -> Value{
  let user =  db.find_user("Interface", "User", id)
  .await
  .expect("User not Found");
  if user.is_none(){
    json!({
      "user": false
  })}
  else{
    json!(user)
  }
}

#[post("/image/<id>", format="image/jpeg", data="<file>")]
pub async fn post_image(id: &str, file: Data<'_>) -> Result<Status, io::Error>{
  let img_path = format!("./temp/image{}.jpg", id);
  let _img_file = fs::File::create(Path::new(img_path.as_str()))?;
  let path: &Path = Path::new(img_path.as_str());
  let data = file.open(2_usize.mebibytes());
  data.into_file(path).await?;
  Ok(Status::new(200))
} 

#[get("/getimg/<id>")]
pub async fn send_image(id: &str) -> Result<NamedFile, Status>{
  let path_str = format!("./temp/image{}.jpg", id);
  let path = Path::new(path_str.as_str());
  if path.exists(){
    Ok(NamedFile::open(path).await.ok().unwrap())
  }
  else{
    Err(Status::new(404))
  }
}