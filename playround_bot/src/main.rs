use std::env;
use std::str;

use futures::StreamExt;
use hyper_rustls::HttpsConnector;
use serde::{Deserialize, Serialize};
use telegram_bot::*;
use std::collections::HashMap;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() {
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let api = Api::new(token);

    // Fetch new updates via long poll method
    let mut stream = api.stream();
    loop {
        if let Some(update) = stream.next().await {
            // If the received update contains a new message...
            match update {
                Err(why) => eprintln!("Receiving update error: {:?}", why),
                Ok(update) => {
                    if let UpdateKind::Message(message) = update.kind {
                        if let MessageKind::Text { ref data, .. } = message.kind {
                            let user = message.from;
                            let msg = {
                                if data == "/start" {
                                    get_start_message()
                                } else if data == "/playground" {
                                    get_playground_url()
                                } else if data == "/github" {
                                    get_github_url()
                                } else if data == "/info" {
                                    get_info(user.id)
                                } else if data.starts_with("/version ") {
                                    set_version(user.id, data.split("/version ").collect())
                                } else if data.starts_with("/mode ") {
                                    set_mode(user.id, data.split("/mode ").collect())
                                } else if data.starts_with("/edition ") {
                                    set_edition(user.id, data.split("/edition ").collect())
                                } else if data.starts_with("/backtrace ") {
                                    set_backtrace(user.id, data.split("/backtrace ").collect())
                                } else if data.starts_with("/cargo ") {
                                    set_build_type(user.id, data.split("/cargo ").collect())
                                } else {
                                    match create_response(user.id, data).await {
                                        Err(why) => {
                                            eprintln!("Create response error: {:?}", why);
                                            "Create response error, sorry for inconvenience".to_string()
                                        }
                                        Ok(response_msg) => {
                                            response_msg
                                        }
                                    }
                                }
                            };

                            match api.send(SendMessage::new(message.chat, msg)).await {
                                Err(why) => {
                                    eprintln!("Send message error: {:?}", why)
                                }
                                Ok(_) => {
                                    ()
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn get_start_message() -> String {
    concat!("Welcome! Let's go deeper to Rust.\n\n",
    "It's Rust Playground Bot.\n",
    "You can check some pieces of your Rust code, sending it to me.\n",
    "I will check it using Rust playground: https://play.rust-lang.org/\n\n",
    "This Bot is an open-source project.\n",
    "https://github.com/vyavdoshenko/rust_playground_bot").to_string()
}

fn get_playground_url() -> String {
    "https://play.rust-lang.org/".to_string()
}

fn get_github_url() -> String {
    "https://github.com/vyavdoshenko/rust_playground_bot".to_string()
}

fn get_info(_user_id: UserId) -> String {
    "".to_string()
}

fn set_version(_user_id: UserId, data: String) -> String {
    data
}

fn set_mode(_user_id: UserId, data: String) -> String {
    data
}

fn set_edition(_user_id: UserId, data: String) -> String {
    data
}

fn set_backtrace(_user_id: UserId, data: String) -> String {
    data
}

fn set_build_type(_user_id: UserId, data: String) -> String {
    data
}

fn get_user_data(_user_id: UserId) -> Option<PlaygroundRequest>
{
    let _users: HashMap<UserId, PlaygroundRequest> = HashMap::new();

    None
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

async fn create_response(_user_id: UserId, data: &str) -> Result<String> {
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
