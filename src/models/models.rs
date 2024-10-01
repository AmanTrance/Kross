use serde::{Serialize, Deserialize};
use crate::uuid::Uuid;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct User{
    #[serde(default="get_id")]
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct SignInUser{
    pub email: String,
    pub password: String
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Arena<'r>{
    pub owner_id: &'r str,
    pub message: String
}

fn get_id() -> String {
    let uuid = Uuid::new_v4().to_string();
    uuid
}