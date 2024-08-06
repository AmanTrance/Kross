extern crate mongodb;
use std::fmt::Debug;
#[allow(unused_imports)]
use mongodb::{results::InsertOneResult, sync::{Client, Collection}};
use serde::{Deserialize, Serialize};

pub struct MongoClient{
    pub client: Client
}

impl MongoClient{
    pub fn init() -> Self{
        const URI: &'static str = "mongodb+srv://amanhobo:ceaHVMMdhMFTeIO5@cluster0.nh1f8qx.mongodb.net/";
        let client = Client::with_uri_str(URI)
        .ok()
        .unwrap();

        MongoClient { client }
    }
    pub async fn create_user<'r, T>(&self,data: T, db_name: &str, collection: &str) -> Result<InsertOneResult, mongodb::error::Error>
    where T: Debug + Serialize + Deserialize<'r> + Send + Sync{
        self.client
        .database(db_name)
        .collection::<T>(collection)
        .insert_one(data)
        .await
    }
}