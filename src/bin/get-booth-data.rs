use booth_logger::commands::booth::save_booth_items;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    save_booth_items().await?;

    Ok(())
}



