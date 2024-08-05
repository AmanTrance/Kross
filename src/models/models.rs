use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct User{
    id: u32,
    name: String,
    age: u32,
    email: String
}