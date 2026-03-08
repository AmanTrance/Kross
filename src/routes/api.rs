use crate::models::models::{Arena, SignInUser, User};
use crate::repository::database::MongoClient;
use rocket::data::{Data, ToByteUnit};
use rocket::fs::NamedFile;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::json::{json, Value};
use rocket::State;
use std::io;
use std::path::Path;

#[get("/")]
pub fn index() -> String {
    String::from("This is a Rust Server")
}

#[post("/signup", format = "application/json", data = "<input>")]
pub async fn user_sign_up(db: &State<MongoClient>, mut input: Json<User>) -> Value {
    if db
        .user_exists("Interface", "User", &input.email, Some(&input.name))
        .await
    {
        json!({
          "data": "username or email already exists"
        })
    } else {
        db.create_user(&mut input, "Interface", "User").await.ok();
        let id: String = db
            .find_user_id("Interface", "User", &input.email)
            .await
            .ok()
            .unwrap()
            .unwrap()
            .id;
        json!({
          "data": {
            "id": id
          }
        })
    }
}

#[post("/signin", format = "application/json", data = "<input>")]
pub async fn user_sign_in(db: &State<MongoClient>, input: Json<SignInUser>) -> Value {
    if db
        .user_exists("Interface", "User", &input.email, None)
        .await
    {
        if db
            .credentials_ok("Interface", "User", &input.email, &input.password)
            .await
        {
            let id = db
                .find_user_id("Interface", "User", &input.email)
                .await
                .ok()
                .unwrap()
                .unwrap()
                .id;
            json!({"id": id})
        } else {
            json!({
              "id": "Wrong Credentials"
            })
        }
    } else {
        json!({
          "id": "Wrong Credentials"
        })
    }
}

#[get("/userdata/<id>")]
pub async fn get_user(db: &State<MongoClient>, id: &str) -> Value {
    let user = db
        .find_user("Interface", "User", id)
        .await
        .expect("Database crashed");
    if user.is_none() {
        json!({
            "user": false
        })
    } else {
        json!(user)
    }
}

#[post("/image/<id>", format = "image/jpeg", data = "<file>")]
pub async fn post_image(id: &str, file: Data<'_>) -> Result<Status, io::Error> {
    file.open(100_usize.megabytes())
        .into_file(format!("./temp/image{}.mkv", id))
        .await?;
    Ok(Status::new(200))
}

#[post("/postvideo/<id>", format = "video/x-matroska", data = "<file>")]
pub async fn upload_video(id: &str, file: Data<'_>) -> Result<Status, io::Error> {
    file.open(100_usize.megabytes())
        .into_file(format!("./temp/video{}.mkv", id))
        .await?;
    Ok(Status::new(200))
}

#[get("/getimg/<id>")]
pub async fn send_image(id: &str) -> Result<NamedFile, Status> {
    let path_str = format!("./temp/image{}.jpg", id);
    let path = Path::new(path_str.as_str());
    if path.exists() {
        Ok(NamedFile::open(path).await.ok().unwrap())
    } else {
        Err(Status::new(404))
    }
}

#[post("/arenapost", format = "application/json", data = "<arena>")]
pub async fn arena_post(db: &State<MongoClient>, arena: Json<Arena<'_>>) -> Status {
    match db
        .create_arena("Interface", "Arena", arena.into_inner())
        .await
    {
        Ok(_) => Status::new(201),
        Err(_) => Status::new(400),
    }
}

#[get("/getarena/<id>?<limit>")]
pub async fn get_arena_details(db: &State<MongoClient>, id: &str, limit: u32) -> Value {
    match db.find_arena("Interface", "Arena", id, limit).await {
        Ok(x) => json!({
          "data": x
        }),
        Err(_) => json!({
          "data": "Arena not filled"
        }),
    }
}
