use mongodb::bson::Document;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Fish {
    id: u32,
    rating: i32,
}
impl Fish {
    pub fn new() {}
}
