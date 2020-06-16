use std::str;

use hyper_rustls::HttpsConnector;
use serde::{Deserialize, Serialize};
use telegram_bot::*;
use std::collections::HashMap;
use std::sync::Mutex;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
type Users = Mutex<HashMap<UserId, PlaygroundRequest>>;

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

pub fn get_info(_user_id: UserId) -> String {
    "".to_string()
}

pub fn set_version(_user_id: UserId, data: String) -> String {
    if data.to_lowercase() == "stable" {
        return "Stable version set.".to_string()
    } else if data.to_lowercase() == "beta" {
        return "Beta version set.".to_string()
    } else if data.to_lowercase() == "nightly" {
        return "Nightly version set.".to_string()
    }

    "Wrong version set.".to_string()
}

pub fn set_mode(_user_id: UserId, data: String) -> String {
    if data.to_lowercase() == "debug" {
        return "Debug mode set.".to_string()
    } else if data.to_lowercase() == "release" {
        return "Release mode set.".to_string()
    }

    "Wrong mode set.".to_string()
}

pub fn set_edition(_user_id: UserId, data: String) -> String {
    if data == "2018" {
        return "2018 edition set.".to_string()
    } else if data == "2015" {
        return "2015 edition set.".to_string()
    }

    "Wrong edition set.".to_string()
}

pub fn set_backtrace(_user_id: UserId, data: String) -> String {
    if data.to_lowercase() == "disabled" {
        return "Disabled backtrace set.".to_string()
    } else if data.to_lowercase() == "enabled" {
        return "Enabled backtrace set.".to_string()
    }

    "Wrong backtrace set.".to_string()
}

pub fn set_build_type(_user_id: UserId, data: String) -> String {
    if data.to_lowercase() == "run" {
        return "run build type set.".to_string()
    } else if data.to_lowercase() == "build" {
        return "build build type set.".to_string()
    } else if data.to_lowercase() == "test" {
        return "test build type set.".to_string()
    } else if data.to_lowercase() == "asm" {
        return "asm build type set.".to_string()
    } else if data.to_lowercase() == "llvm ir" {
        return "llvm ir build type set.".to_string()
    } else if data.to_lowercase() == "mir" {
        return "mir build type set.".to_string()
    } else if data.to_lowercase() == "wasm" {
        return "wasm build type set.".to_string()
    }

    "Wrong build type set.".to_string()
}

fn load_users_data(_file_path: String) -> Users
{
    Mutex::new(HashMap::new())
}

#[allow(non_snake_case)]
#[derive(Serialize)]
struct PlaygroundRequest {
    backtrace: bool,
    channel: String,
    code: String,
    crateType: String,
    edition: String,
    mode: String,
    tests: bool,
}

#[derive(Deserialize)]
struct PlaygroundResponse {
    success: bool,
    stdout: String,
    stderr: String,
}

pub async fn create_response(_user_id: UserId, data: &str) -> Result<String> {
    let connector = HttpsConnector::new();
    let client = hyper::Client::builder().build(connector);

    let playground_request = serde_json::to_string(&PlaygroundRequest {
        backtrace: false,
        channel: String::from("stable"),
        code: data.to_string(),
        crateType: String::from("bin"),
        edition: String::from("2018"),
        mode: String::from("debug"),
        tests: false,
    })?;

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
