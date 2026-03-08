mod models;
mod repository;
mod routes;
#[macro_use]
extern crate rocket;
extern crate uuid;
use repository::database::MongoClient;
use rocket::http::Method;
use rocket_cors::{AllowedOrigins, CorsOptions};
use routes::api::{
    arena_post, get_arena_details, get_user, index, post_image, send_image, upload_video,
    user_sign_in, user_sign_up,
};

#[launch]
fn rocket() -> _ {
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![
                Method::Get,
                Method::Post,
                Method::Patch,
                Method::Delete,
                Method::Put,
            ]
            .into_iter()
            .map(From::from)
            .collect(),
        )
        .allow_credentials(true);

    let db = MongoClient::init();

    rocket::build()
        .manage(db)
        .attach(cors.to_cors().unwrap())
        .mount("/api", routes![index, user_sign_up, user_sign_in, get_user])
        .mount(
            "/api",
            routes![
                post_image,
                upload_video,
                send_image,
                arena_post,
                get_arena_details
            ],
        )
}
