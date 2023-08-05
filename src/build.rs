use std::fs;
use std::path::Path;

fn main() -> std::io::Result<()> {
    if !Path::new("./data").exists() {
        fs::create_dir("./data")?;
    }
    Ok(())
}



