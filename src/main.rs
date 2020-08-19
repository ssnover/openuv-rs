use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut key_file = File::open("api-key.json")?;
    let mut contents = String::new();
    key_file.read_to_string(&mut contents)?;
    let parsed_key_file: serde_json::Value = serde_json::from_str(&contents)?;
    let key = &parsed_key_file["api_key"].as_str().unwrap();

    Ok(())
}
