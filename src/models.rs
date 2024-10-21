use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Item {
    pub id: u32,
    pub name: String,
}

#[derive(Serialize,Deserialize)]
pub struct RedisItem {
   pub key: String,
   pub value: String,
}