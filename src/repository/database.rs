extern crate mongodb;

use mongodb::sync::{Client, Collection};
use rocket::{State};

use crate::User;

struct Mongo{
    collection: Collection<User>
}