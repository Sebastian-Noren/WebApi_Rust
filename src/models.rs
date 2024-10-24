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


// Define a local Book struct for JSON serialization
#[derive(Debug,Serialize, Deserialize)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub pages: i32,
}

#[derive(Debug, Serialize)]
pub struct BookA{
    pub title: String,
    pub author: String,
    pub pages: u32,
    pub description: Option<String>
}

impl Default for BookA{
    fn default() -> Self {
        Self{
            title: "Untitled".to_string(),
            author: "".to_string(),
            pages: 0,
            description: None,
        }
    }
}