#[macro_use]
extern crate serde_derive;

use std::fs;
use serde_json::Result;

#[derive(Deserialize)]
struct Config {
    jira_url: String,
    jira_token: String,
    jira_project: String,
}

fn main() -> Result<()> {
    let file_path = "/opt/kujira/config.json";
    let data = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let config: Config = serde_json::from_str(&data)?;

    println!("{:#?}", config.jira_url);
    println!("{:#?}", config.jira_token);
    println!("{:#?}", config.jira_project);

    Ok(())
}
