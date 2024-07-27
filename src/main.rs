#[macro_use] extern crate rocket;
#[allow(unused_imports)]
use rocket::Data;

#[get("/")]
async fn index() -> String{
    String::from("Hello World")
}

#[launch]
async fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}