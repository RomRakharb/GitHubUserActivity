use reqwest::header::{HeaderMap, ACCEPT, AUTHORIZATION, USER_AGENT};
use serde_json::Value;
use std::env;
use std::fs::File;
use std::io::Read;

enum Action {
    Get(String),
    SetToken(String),
}

fn get_token() -> std::io::Result<String> {
    let mut token = String::new();
    let mut file = File::open("token")?;
    file.read_to_string(&mut token)?;
    Ok(token)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args();
    let username = args.nth(1).unwrap();
    let binding = get_token()?;
    let token = binding.trim_end_matches('\n');

    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, "application/vnd.github+json".parse()?);
    headers.insert(AUTHORIZATION, format!("Bearer {token}").parse()?);
    headers.insert("X-GitHub-Api-Version", "2022-11-28".parse()?);
    headers.insert(USER_AGENT, "GitHub User Activity".parse()?);

    let client = reqwest::blocking::Client::new();

    let response = client
        .get(format!("https://api.github.com/users/{}/events", username))
        .headers(headers)
        .send()?;

    if response.status().is_success() {
        let body: Value = serde_json::from_str(response.text().unwrap().as_str())?;
        if let Some(each) = body.as_array() {
            println!("{:?}", each);
        }
        // println!("{:?}", body);
    } else {
        println!("Error: {}", response.status());
        let error_body = response.text()?;
        println!("Response body: {}", error_body);
    }
    Ok(())
}
