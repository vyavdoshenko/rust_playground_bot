use std::collections::HashMap;
use std::str;
use std::sync::Mutex;

use hyper_rustls::HttpsConnector;
use serde::{Deserialize, Serialize};
use telegram_bot::*;

// Private section

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Deserialize)]
struct PlaygroundResponse {
    success: bool,
    stdout: String,
    stderr: String,
}

fn get_user_data(user_id: UserId, users: &Users) -> PlaygroundRequest {
    let locker = users.lock().unwrap();
    if locker.contains_key(&user_id) {
        return locker.get(&user_id).unwrap().clone();
    }

    PlaygroundRequest::new()
}

// Public section

pub type Users = Mutex<HashMap<UserId, PlaygroundRequest>>;

#[allow(non_snake_case)]
#[derive(Serialize, Clone)]
pub struct PlaygroundRequest {
    backtrace: bool,
    channel: String,
    code: String,
    crateType: String,
    edition: String,
    mode: String,
    tests: bool,
}

impl PlaygroundRequest {
    pub fn new() -> PlaygroundRequest {
        PlaygroundRequest {
            backtrace: false,
            channel: String::from("stable"),
            code: String::from(""),
            crateType: String::from("bin"),
            edition: String::from("2018"),
            mode: String::from("debug"),
            tests: false,
        }
    }
}

pub fn get_start_message() -> String {
    concat!("Welcome! Let's go deeper to Rust.\n\n",
    "It's Rust Playground Bot.\n",
    "You can check some pieces of your Rust code, sending it to me.\n",
    "I will check it using Rust playground: https://play.rust-lang.org/\n\n",
    "This Bot is an open-source project.\n",
    "https://github.com/vyavdoshenko/rust_playground_bot").to_string()
}

pub fn get_playground_url() -> String {
    "https://play.rust-lang.org/".to_string()
}

pub fn get_github_url() -> String {
    "https://github.com/vyavdoshenko/rust_playground_bot".to_string()
}

pub fn get_info(user_id: UserId, users: &Users) -> String {
    let data = get_user_data(user_id, users);

    let mut value = "Backtrace: ".to_string();
    if data.backtrace {
        value.push_str("enabled\n");
    } else {
        value.push_str("disabled\n");
    }

    value.push_str("Channel: ");
    value.push_str(&data.channel);

    value.push_str("\nEdition: ");
    value.push_str(&data.edition);

    value.push_str("\nMode: ");
    value.push_str(&data.mode);

    value.push_str("\nTests: ");
    if data.tests {
        value.push_str("enabled");
    } else {
        value.push_str("disabled");
    }

    value
}

pub fn set_version(_user_id: UserId, data: String) -> String {
    if data.to_lowercase() == "stable" ||
        data.to_lowercase() == "beta" ||
        data.to_lowercase() == "nightly" {
        data.to_lowercase().clone().push_str(" version set.")
    }

    "Wrong version set.".to_string()
}

pub fn set_mode(_user_id: UserId, data: String) -> String {
    if data.to_lowercase() == "debug" || data.to_lowercase() == "release" {
        data.to_lowercase().clone().push_str(" mode set.")
    }

    "Wrong mode set.".to_string()
}

pub fn set_edition(_user_id: UserId, data: String) -> String {
    if data == "2018" || data == "2015" {
        data.clone().push_str(" edition set.")
    }

    "Wrong edition set.".to_string()
}

pub fn set_backtrace(_user_id: UserId, data: String) -> String {
    if data.to_lowercase() == "disabled" || data.to_lowercase() == "enabled" {
        data.to_lowercase().clone().push_str(" backtrace set.")
    }

    "Wrong backtrace set.".to_string()
}

pub fn set_build_type(_user_id: UserId, data: String) -> String {
    if data.to_lowercase() == "run" ||
        data.to_lowercase() == "build" ||
        data.to_lowercase() == "test" ||
        data.to_lowercase() == "asm" ||
        data.to_lowercase() == "llvm ir" ||
        data.to_lowercase() == "mir" ||
        data.to_lowercase() == "wasm" {
        data.to_lowercase().clone().push_str(" build type set.")
    }

    "Wrong build type set.".to_string()
}

pub fn load_users_data(_file_path: String) -> Users
{
    Mutex::new(HashMap::new())
}

pub async fn create_response(user_id: UserId, users: &Users, data: &str) -> Result<String> {
    let connector = HttpsConnector::new();
    let client = hyper::Client::builder().build(connector);

    let mut user_data = get_user_data(user_id, users);
    user_data.code = data.to_string();

    let playground_request = serde_json::to_string(&user_data)?;

    let req = hyper::Request::post("https://play.rust-lang.org/execute")
        .header("content-type", "application/json")
        .body(hyper::Body::from(playground_request))?;

    let body = client.request(req).await?;
    let bytes = hyper::body::to_bytes(body).await?;

    let playground_response: PlaygroundResponse = serde_json::from_slice(&bytes[..])?;

    let mut value = "---- Standard Error ----\n\n".to_string();
    value.push_str(playground_response.stderr.as_str());
    if playground_response.success {
        value.push_str("\n---- Standard Output ----\n\n");
        value.push_str(playground_response.stdout.as_str());
    }

    Ok(value)
}
