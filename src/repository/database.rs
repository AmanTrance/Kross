extern crate mongodb;
extern crate dotenv;

use mongodb::bson::doc;
use mongodb::{results::InsertOneResult, sync::Client};
use dotenv::dotenv;
use std::env;

use crate::models::models::User;

pub struct MongoClient{
    pub client: Client
}

impl MongoClient{
    pub fn init() -> Self{
        dotenv().ok();
        let db_key: String = env::var("MONGO_URI").unwrap();
        let uri: String = db_key;
        let client = Client::with_uri_str(uri)
        .ok()
        .unwrap();

        MongoClient { client }
    }
    pub async fn create_user(&self, data: User, db_name: &str, collection: &str) -> Result<InsertOneResult, mongodb::error::Error>{
        self.client
        .database(db_name)
        .collection::<User>(collection)
        .insert_one(data)
        .await
    }
    pub async fn find_user(&self, db_name: &str, collection: &str, id: String) -> Result<Option<User>, mongodb::error::Error>{
        self.client
        .database(db_name)
        .collection::<User>(collection)
        .find_one(doc!{"id": id})
        .await
    }
    pub async fn find_user_id(&self, db_name: &str, collection: &str, email: String) -> Result<Option<User>, mongodb::error::Error>{
        self.client
        .database(db_name)
        .collection::<User>(collection)
        .find_one(doc!{"email": email})
        .await
    }
    pub async fn user_exists(&self, db_name: &str, collection: &str, email: String) -> bool{
        match self.client.database(db_name).collection::<User>(collection).find_one(doc!{
            "email": email
        }).await{
            Ok(Some(_)) => true,
            Ok(None) => false,
            Err(_) => false
        }
    }
    pub async fn credentials_ok(&self, db_name: &str, collection: &str, name: String, email: String, password: String) -> bool{
        match self.client.database(db_name).collection::<User>(collection).find_one(doc!{
            "name": name,
            "email": email,
            "password": password
        }).await{
            Ok(Some(_)) => true,
            Ok(None) => false,
            Err(_) => false
        }
    }
}