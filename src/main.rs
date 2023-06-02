//! Main file

use clap::Parser;
use humansize::{format_size, DECIMAL};
use reqwest::blocking::Client;
use serde_json::Value;

static GITHUB_REPOSITORY_BASE_URL: &str = "https://api.github.com/repos/";

fn github_api_get_metadata(target_url: String) -> String {
    let url = GITHUB_REPOSITORY_BASE_URL.to_string() + &target_url;
    let client = Client::new();
    let raw_metadata = client
        .get(url)
        .header("User-Agent", "squiddy")
        .send()
        .expect("Failed to send message to GitHub API. Try again");

    return raw_metadata.text().unwrap();
}

fn github_api_get_repository_size(target: String) -> String {
    let text_data = github_api_get_metadata(target);

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

    github_api_get_repository_size(commands.repository);
}
