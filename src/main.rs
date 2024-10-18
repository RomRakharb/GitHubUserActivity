use reqwest::header::{ACCEPT, AUTHORIZATION};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args();
    let username = args.nth(1).unwrap();

    let body = reqwest::blocking::get(format!("https://api.github.com/users/{}/events", username))?
        .text()?;

    println!("body = {body:?}");
    Ok(())
}
