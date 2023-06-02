//! Main file

use clap::Parser;
use humansize::{format_size, DECIMAL};
use serde_json::Value;

/// Gets and parses metadata from the 'target' repository
fn get_size_from(target: String) -> String {
    let url = "https://api.github.com/repos/".to_string() + &target;
    let client = reqwest::blocking::Client::new();

    let raw_data = client
        .get(url)
        .header("User-Agent", "squiddy")
        .send()
        .expect("Failed to send message to GitHub API. Try again");

    let text_data = raw_data.text().unwrap();

    let data: Value = serde_json::from_str(&text_data).expect("Error converting data");

    let raw_size: u64 = data["size"]
        .clone()
        .as_u64()
        .expect("Erro extracting size from JSON data")
        * 1000;

    let formated: String = format_size(raw_size, DECIMAL);

    println!("Repository size: {}", formated);

    formated
}

#[derive(Parser)]
#[command(about = "Squiddy - GitHub analytics tool")]
struct Commands {
    /// Repository name(in <author>/<repository> form)
    repository: String,
}

fn main() {
    let commands = Commands::parse();

    get_size_from(commands.repository);
}
