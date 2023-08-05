use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub price: i32,
    pub url: String,
    pub id: i32,
    pub favorite: i32,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ItemList {
    pub date: String,
    pub items: Vec<Item>,
}