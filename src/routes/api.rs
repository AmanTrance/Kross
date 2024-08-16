use crate::repository::database::MongoClient;
use crate::models::models::{Arena, User};
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
  if db.user_exists("Interface", "User", &input.email).await {
    if db.credentials_ok("Interface", "User", &input.name, &input.email, &input.password).await{
      let id = db
      .find_user_id("Interface", "User", &input.email)
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
    .create_user(&input, "Interface", "User")
    .await
    .ok();
    let id = db
    .find_user_id("Interface", "User", &input.email)
    .await
    .ok()
    .unwrap()
    .unwrap()
    .id;
    json!({"id": id})
  }
}

#[get("/userdata/<id>")]
pub async fn get_user(db: &State<MongoClient>, id: &str) -> Value{
  let user =  db
  .find_user("Interface", "User", id)
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

#[post("/arenapost", format="application/json", data="<arena>")]
pub async fn arena_post(db: &State<MongoClient>, arena: Json<Arena<'_>>) -> Status{
  match db.create_arena("Interface", "Arena", &arena).await{
    Ok(_) => Status::new(201),
    Err(_) => Status::new(400)
  }
}

#[get("/getarena/<id>")]
pub async fn get_arena_details(db: &State<MongoClient>, id: &str) -> Value{
  match db.find_arena("Interface", "Arena", id).await{
    Ok(x) => json!({
      "data": x
    }),
    Err(_) => json!({
      "data": "Arena not filled"
    })
  }
}
