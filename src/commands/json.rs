use crate::models::item::ItemList;

pub fn save_json(items: ItemList, filepath: &str) {
    //let json = serde_json::to_string(&items).unwrap();
    let file = std::fs::File::create(filepath).unwrap();
    serde_json::to_writer_pretty(file, &items).unwrap();
}

pub fn read_json(filepath: &str) -> ItemList {
    let file = std::fs::File::open(filepath).unwrap();
    let reader = std::io::BufReader::new(file);
    let itemlist: ItemList = serde_json::from_reader(reader).unwrap();
    itemlist
}