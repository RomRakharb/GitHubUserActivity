use reqwest::header::{HeaderMap, ACCEPT, AUTHORIZATION, USER_AGENT};
use serde_json::Value;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args();
    let username = args.nth(1).unwrap();
    let token = "";

    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, "application/vnd.github+json".parse()?);
    headers.insert(AUTHORIZATION, format!("Bearer {}", token).parse()?);
    headers.insert("X-GitHub-Api-Version", "2022-11-28".parse()?);
    headers.insert(USER_AGENT, "GitHub ROM".parse()?);

    let client = reqwest::blocking::Client::new();
    let response = client
        .get(format!("https://api.github.com/users/{}/events", username))
        .headers(headers)
        .send()?;

    // Check if the response was successful
    if response.status().is_success() {
        let body: Value = response.json()?;
        let pretty_body = serde_json::to_string_pretty(&body)?;
        println!("{}", pretty_body);
    } else {
        println!("Error: {}", response.status());
        let error_body = response.text()?;
        println!("Response body: {}", error_body);
    }
    Ok(())
}
