#[macro_use]
extern crate serde_derive;

use std::fs;
use serde_json::Result;

#[derive(Deserialize, Serialize)]
struct Config {
    jiraUrl: String,
    jiraToken: String,
}

fn main() -> Result<()> {
    let file_path = "/opt/kujira/config.json";
    let data = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let config: Config = serde_json::from_str(&data)?;

    println!("{:#?}", config.jiraUrl);

    Ok(())
}
