use reqwest::header::{ACCEPT, AUTHORIZATION};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let username = args.get(1).unwrap();

    let body = reqwest::blocking::get(format!("https://api.github.com/users/{}/events", username))?
        .text()?;

    println!("body = {body:?}");
    Ok(())
}
