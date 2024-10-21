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

#[derive(Debug, Serialize)]
pub struct Book{
    pub title: String,
    pub author: String,
    pub pages: u32,
    pub description: Option<String>
}

impl Default for Book{
    fn default() -> Self {
        Self{
            title: "Untitled".to_string(),
            author: "".to_string(),
            pages: 0,
            description: None,
        }
    }
}