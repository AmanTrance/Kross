use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct User{
    pub id: u32,
    pub name: String,
    pub age: u32,
    pub email: String
}