use booth_logger::commands::json::{save_json, read_json};
use booth_logger::models::item::{Item, ItemList};
use booth_logger::commands::booth::{get_booth_items, save_booth_items};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    save_booth_items().await?;

    Ok(())
}



