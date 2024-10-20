use reqwest::header::{HeaderMap, ACCEPT, AUTHORIZATION, USER_AGENT};
use serde_json::Value;
use std::env;
use token::{get_token, set_token};

mod token {
    use std::fs::File;
    use std::io::{Read, Write};
    pub fn get_token() -> std::io::Result<String> {
        let mut token = String::new();
        let mut file = File::open("token")?;
        file.read_to_string(&mut token)?;
        token = token.trim_end_matches('\n').to_string();
        Ok(token)
    }

    pub fn set_token(token: String) -> std::io::Result<()> {
        let mut file = File::create("token")?;
        file.write_all(token.as_bytes())?;
        Ok(())
    }
}

enum Activity {
    Push(String, u32),
    Create(String),
    Star(String),
    None,
}

struct Activities(Vec<Activity>);

impl Activities {
    fn process(&self) {
        for activity in &self.0 {
            match activity {
                Activity::Push(repo, commit) => println!("Pushed {} commit(s) to {}", commit, repo),
                Activity::Create(repo) => println!("Created {}", repo),
                Activity::Star(repo) => println!("Starred {}", repo),
                Activity::None => {}
            }
        }
    }
}

fn get_activity(username: String, token: String) -> Result<(), Box<dyn std::error::Error>> {
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

    let body: Value = serde_json::from_str(&response.text()?)?;

    let mut activities: Activities = Activities(Vec::new());

    if let Some(events) = body.as_array() {
        for event in events {
            let mut tmp_action = String::new();
            let mut tmp_repo = String::new();
            let mut tmp_commit: u32 = 0;

            if let Some(action) = event.get("type").and_then(Value::as_str) {
                tmp_action = action.to_string();
            }

            if let Some(repo) = event.get("repo").and_then(Value::as_object) {
                if let Some(name) = repo.get("name").and_then(Value::as_str) {
                    tmp_repo = name.to_string();
                }
            }

            if let Some(payload) = event.get("payload").and_then(Value::as_object) {
                if let Some(size) = payload.get("size").and_then(Value::as_u64) {
                    tmp_commit = size as u32;
                }
            }

            let activity = match tmp_action.as_str() {
                "PushEvent" => Activity::Push(tmp_repo, tmp_commit),
                "CreateEvent" => Activity::Create(tmp_repo),
                "WatchEvent" => Activity::Star(tmp_repo),
                _ => Activity::None,
            };

            activities.0.push(activity);
        }
    }

    activities.process();
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args();
    match (args.nth(1), args.nth(2)) {
        (Some(username), None) => get_activity(username, get_token()?)?,
        (Some(ref action), Some(token)) if action == "token" => set_token(token)?,
        _ => {}
    };

    Ok(())
}
