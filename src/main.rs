mod repository;
mod routes;
mod models;
use repository::database::MongoClient;
use rocket::http::Method;
use rocket_cors::{AllowedOrigins, CorsOptions};
use routes::api::{index, name, user, userdata, post_image, send_image};
#[macro_use] extern crate rocket;

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
