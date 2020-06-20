use std::collections::HashMap;
use std::str;
use std::sync::Mutex;

use hyper_rustls::HttpsConnector;
use serde::{Deserialize, Serialize};
use telegram_bot::*;
use std::io::{BufReader, BufWriter};
use std::fs::File;
use std::ops::Deref;

// Private section

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Deserialize)]
struct PlaygroundResponse {
    success: bool,
    stdout: String,
    stderr: String,
}

fn get_user_data(user_id: UserId, users: &Users) -> PlaygroundRequest {
    let guard = users.lock().unwrap();
    if guard.contains_key(&user_id) {
        return guard.get(&user_id).unwrap().clone();
    }

    PlaygroundRequest::new()
}

// Public section

pub type Users = Mutex<HashMap<UserId, PlaygroundRequest>>;

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Clone)]
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

    value.push_str("\nCrate type: ");
    value.push_str(&data.crateType);

    value.push_str("\nTests: ");
    if data.tests {
        value.push_str("enabled");
    } else {
        value.push_str("disabled");
    }

    value
}

pub fn set_channel(user_id: UserId, users: &mut Users, data: String) -> String {
    println!("Set channel {:?} for user {:?}", data, user_id);
    if data.to_lowercase() == "stable" ||
        data.to_lowercase() == "beta" ||
        data.to_lowercase() == "nightly" {
        let mut user_data = get_user_data(user_id, users);

        let mut value = "".to_string();
        value.push_str("Channel is ");
        value.push_str(&data);

        user_data.channel = data;

        let mut guard = users.lock().unwrap();
        guard.insert(user_id, user_data);

        return value;
    }

    "Error. Wrong channel.".to_string()
}

pub fn set_mode(user_id: UserId, users: &mut Users, data: String) -> String {
    println!("Set mode {:?} for user {:?}", data, user_id);
    if data.to_lowercase() == "debug" || data.to_lowercase() == "release" {
        let user_data = get_user_data(user_id, users);

        let mut value = "".to_string();
        value.push_str("Mode is ");
        value.push_str(&data);

        let mut guard = users.lock().unwrap();
        guard.insert(user_id, user_data);

        return value;
    }

    "Error. Wrong mode.".to_string()
}

pub fn set_edition(user_id: UserId, users: &mut Users, data: String) -> String {
    println!("Set edition {:?} for user {:?}", data, user_id);
    if data == "2018" || data == "2015" {
        let user_data = get_user_data(user_id, users);

        let mut value = "".to_string();
        value.push_str("Edition is ");
        value.push_str(&data);

        let mut guard = users.lock().unwrap();
        guard.insert(user_id, user_data);

        return value;
    }

    "Error. Wrong edition.".to_string()
}

pub fn set_backtrace(user_id: UserId, users: &mut Users, data: String) -> String {
    println!("Set backtrace {:?} for user {:?}", data, user_id);
    if data.to_lowercase() == "disabled" || data.to_lowercase() == "enabled" {
        let mut user_data = get_user_data(user_id, users);

        if data.to_lowercase() == "disabled" {
            user_data.backtrace = false;
        } else {
            user_data.backtrace = true;
        }

        let mut guard = users.lock().unwrap();
        guard.insert(user_id, user_data);

        let mut value = "".to_string();
        value.push_str("Backtrace ");
        value.push_str(&data);
        return value;
    }

    "Error. Wrong backtrace.".to_string()
}

pub fn set_build_type(user_id: UserId, users: &mut Users, data: String) -> String {
    println!("Set build type {:?} for user {:?}", data, user_id);
    if data.to_lowercase() == "run" ||
        data.to_lowercase() == "build" ||
        data.to_lowercase() == "test" {
        let mut user_data = get_user_data(user_id, users);

        if data.to_lowercase() == "test" {
            user_data.tests = true;
        } else {
            user_data.tests = false;
        }

        if data.to_lowercase() == "run" {
            user_data.crateType = "bin".to_string();
        } else {
            user_data.crateType = "lib".to_string();
        }

        let mut guard = users.lock().unwrap();
        guard.insert(user_id, user_data);

        let mut value = "".to_string();
        value.push_str(&data);
        value.push_str(" build type set.");
        return value;
    }

    "Error. Wrong build type.".to_string()
}

pub fn load_users_data(file_path: &String) -> Users
{
    match File::open(file_path) {
        Err(_) => {
            Mutex::new(HashMap::new())
        }
        Ok(file) => {
            let reader = BufReader::new(file);

            match serde_json::from_reader(reader) {
                Err(_) => {
                    Mutex::new(HashMap::new())
                }
                Ok(all) => {
                    Mutex::new(all)
                }
            }
        }
    }
}

pub fn save_users_data(file_path: &String, users : &Users) {
    match File::create(&file_path) {
        Err(_) => {
            eprintln!("Can't open file for save: {:?}", file_path);
        }
        Ok(file) => {
            let writer = BufWriter::new(file);
            let guard = users.lock().unwrap();
            match serde_json::to_writer(writer, guard.deref()) {
                Err(_) => {
                    eprintln!("Can't save file: {:?}", file_path);
                }
                Ok(_) => {
                }
            }
        }
    }
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
