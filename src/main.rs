use reqwest::header::{HeaderMap, ACCEPT, AUTHORIZATION, USER_AGENT};
use serde_json::Value;
use std::env;
use std::fs::File;
use std::io::{Read, Write};

enum Action {
    Get(String),
    SetToken(String),
}

fn get_token() -> std::io::Result<String> {
    let mut token = String::new();
    let mut file = File::open("token")?;
    file.read_to_string(&mut token)?;
    token = token.trim_end_matches('\n').to_string();
    Ok(token)
}

fn set_token(token: String) -> std::io::Result<()> {
    let mut file = File::create("token")?;
    file.write_all(token.as_bytes())?;
    Ok(())
}

fn get_activity(username: String, token: String) -> Result<String, Box<dyn std::error::Error>> {
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

    Ok(response.text()?)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args();
    let mut response = String::new();
    match args.nth(1) {
        Some(arg) => match arg.as_str() {
            "token" => set_token(args.nth(2).unwrap_or_default())?,
            _ => response = get_activity(arg, get_token()?)?,
        },
        None => {}
    }

    Ok(())
}
