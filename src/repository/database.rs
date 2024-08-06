extern crate mongodb;

use mongodb::{bson::extjson::de::Error, results::InsertOneResult, sync::{Client, Collection}};
use rocket::{State};

use crate::User;

pub struct MongoClient{
    pub collection: Collection<User>
}

impl MongoClient{
    pub fn init() -> Self{
        const URI: &'static str = "mongodb+srv://amanhobo:ceaHVMMdhMFTeIO5@cluster0.nh1f8qx.mongodb.net/";
        let client = Client::with_uri_str(URI)
        .ok()
        .unwrap();

        let db = client.database("Interface");
        let collect: Collection<User> = db.collection("User");
        MongoClient { collection: collect }
    }
    pub async fn create_user(&self, user: User) -> Result<InsertOneResult, mongodb::error::Error>{
        self.collection.insert_one(user).await
    }
}