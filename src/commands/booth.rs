use scraper::Html;
use scraper::Selector;
use thirtyfour::prelude::*;
use chrono::Local;
use std::env;
use dotenv::dotenv;

use crate::models::item::{Item, ItemList};
use crate::commands::json::{save_json, read_json};

pub async fn save_booth_items() -> Result<(), Box<dyn std::error::Error>>{
    dotenv().ok();
    let today = Local::now().format("%Y-%m-%d").to_string();
    let shop_url = env::var("SHOP_URL").unwrap();
    let shop_name = env::var("SHOP_NAME").unwrap();
    let filepath = format!("data/{}-{}.json", shop_name, today);

    let items = get_booth_items(&shop_url).await?;
    
    let itemlist = ItemList {
        date: today,
        items: items,
    };

    save_json(itemlist, &filepath);
    Ok(())
}

pub async fn get_booth_items(shop_url: &str) -> Result<Vec<Item>, Box<dyn std::error::Error>> {
    let mut page = 1;

    let caps = DesiredCapabilities::chrome();
    let selenium_url = std::env::var("SELENIUM_URL").unwrap();
    let driver = WebDriver::new(&selenium_url, caps).await?;

    let mut items: Vec<Item> = Vec::new();

    // ページ数だけ繰り返す
    loop {
        let url = format!("{}/items?page={}", shop_url, page);

        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        driver.goto(url).await?;

        let result = driver.source().await;
        match result {
            Ok(result) => {
                let res = try_parse_html(&result, shop_url);
                if res.len() == 0 {
                    break;
                } else {
                    items.extend(res);
                }
            },
            Err(_) => break,
        }
        println!("page: {}", page);
        page += 1;
    }
    println!("total pages: {}", page - 1);
    //println!("items: {:?}", items);

    driver.quit().await?;

    Ok(items)
}

fn try_parse_html(html: &str, shop_url: &str) -> Vec<Item> {
    let document = Html::parse_document(html);

    let name_selector = Selector::parse("h2.item-name").unwrap();
    let favorite_selector = Selector::parse("span.count").unwrap();
    let id_selector = Selector::parse("li.item").unwrap();

    let mut name_vec: Vec<String> = Vec::new();
    let mut price_vec: Vec<i32> = Vec::new();
    let mut favorite_vec: Vec<i32> = Vec::new();
    let mut id_vec: Vec<i32> = Vec::new();

    let mut item_vec: Vec<Item> = Vec::new();


    // name
    for element in document.select(&name_selector) {
        if let Some(name) = element.text().next() {
            name_vec.push(name.to_string());
        }
    }

    // favorite
    for element in document.select(&favorite_selector) {
        if let Some(favorite) = element.text().next() {
            favorite_vec.push(favorite.parse::<i32>().unwrap());
            
        }
    }

    // id, price, (url)
    for element in document.select(&id_selector) {
        if let Some(id) = element.value().attr("data-product-id") {
            id_vec.push(id.parse::<i32>().unwrap());
        }
        if let Some(price) =  { element.value().attr("data-product-price") } {
            price_vec.push(price.parse::<i32>().unwrap());
            
        }
    }

    // Item構造体に格納
    for i in 0..name_vec.len() {
        let item = Item {
            name: name_vec[i].clone(),
            price: price_vec[i],
            url: format!("{}/items/{}", shop_url, id_vec[i].to_string()),
            id: id_vec[i],
            favorite: favorite_vec[i],
        };
        item_vec.push(item);
    }

    item_vec
}