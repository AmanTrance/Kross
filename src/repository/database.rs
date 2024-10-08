use mongodb::bson::{doc, Document};
use mongodb::{results::InsertOneResult, sync::Client};
use dotenv::dotenv;
use rocket::futures::TryStreamExt;
use std::env;
use bcrypt;

use crate::models::models::{Arena, User};

pub struct MongoClient{
    pub client: Client
}

impl MongoClient{
    pub fn init() -> Self {
        dotenv().ok();
        let db_key: String = env::var("MONGO_URI").unwrap();
        let uri: String = db_key;
        let client: Client = Client::with_uri_str(uri).ok().unwrap();

        MongoClient { client }
    }
    pub async fn create_user(&self, data: &mut User, db_name: &str, collection: &str) -> Result<InsertOneResult, mongodb::error::Error> {
        let hashed_password: String = bcrypt::hash(data.password.clone(), 14).unwrap();
        data.password = hashed_password;
        self.client
        .database(db_name)
        .collection::<User>(collection)
        .insert_one(data)
        .await
    }
    pub async fn find_user(&self, db_name: &str, collection: &str, id: &str) -> Result<Option<User>, mongodb::error::Error> {
        self.client
        .database(db_name)
        .collection::<User>(collection)
        .find_one(doc!{"id": id})
        .await
    }
    pub async fn find_user_id(&self, db_name: &str, collection: &str, email: &str) -> Result<Option<User>, mongodb::error::Error> {
        self.client
        .database(db_name)
        .collection::<User>(collection)
        .find_one(doc!{"email": email})
        .await
    }
    pub async fn user_exists(&self, db_name: &str, collection: &str, email: &str, name: Option<&str>) -> bool {
        if name.is_none() {
            match self.client.database(db_name).collection::<User>(collection).find_one(doc!{    
                "email": email
            }).await {
                Ok(Some(_)) => true,
                Ok(None) => false,
                Err(_) => false
            }    
        } else {
            match self.client.database(db_name).collection::<User>(collection).find_one(doc!{
                "name": name.unwrap(),    
                "email": email
            }).await {
                Ok(Some(_)) => true,
                Ok(None) => false,
                Err(_) => false
            }    
        }
    }
    pub async fn credentials_ok(&self, db_name: &str, collection: &str, email: &str, password: &str) -> bool {
        match self.client.database(db_name).collection::<User>(collection).find_one(doc!{
            "email": email
        }).await {
            Ok(Some(user)) => {
                match bcrypt::verify(password, &user.password) {
                    Ok(ans) => {
                        if ans == true {
                            true
                        } else {
                            false
                        }
                    },
                    Err(_) => false
                }
            },
            Ok(None) => false,
            Err(_) => false
        }
    }
    pub async fn create_arena(&self, db_name: &str, collection: &str, data: Arena<'_>) -> Result<InsertOneResult, mongodb::error::Error> {
        self
        .client
        .database(db_name)
        .collection::<Arena>(collection)
        .insert_one(data)
        .await
    }
    pub async fn find_arena(&self, db_name: &str, collection: &str, id: &str, limit: u32) -> Result<Vec<Document>, mongodb::error::Error> {
        match self
        .client
        .database(db_name)
        .collection::<Arena>(collection)
        .aggregate([
            doc! {
                "$sort": {
                    "_id": -1
                }
            },
            doc! {
                "$match": {
                    "owner_id": {
                        "$ne": id
                    }
                }
            }, 
            doc! {
                "$limit": limit
            }
            ]).await {
                Ok(x) => x.try_collect::<Vec<Document>>().await,
                Err(_) => Ok(Vec::<Document>::new())
            }
    }
}