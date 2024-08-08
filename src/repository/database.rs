extern crate mongodb;

use mongodb::bson::doc;
use mongodb::{results::InsertOneResult, sync::Client};

use crate::models::models::User;

pub struct MongoClient{
    pub client: Client
}

impl MongoClient{
    pub fn init() -> Self{
        const URI: &'static str = "mongodb+srv://amanhobo:eaHVMMdhMFTeIO5@cluster0.nh1f8qx.mongodb.net/";
        let client = Client::with_uri_str(URI)
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
    pub async fn find_user(&self, db_name: &str, collection: &str, id: u32) -> Result<std::option::Option<User>, mongodb::error::Error>{
        self.client
        .database(db_name)
        .collection::<User>(collection)
        .find_one(doc!{"id": id})
        .await
    }
}