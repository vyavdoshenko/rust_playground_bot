use std::env;

use futures::StreamExt;
use hyper_rustls::HttpsConnector;
use serde::{Serialize, Deserialize};
use telegram_bot::*;

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
                            let msg;
                            if data == "/start" {
                                msg = get_start_message();
                            } else if data == "/playground" {
                                    msg = get_playground_url();
                            } else if data == "/github" {
                                msg = get_github_url();
                            } else {
                                match create_response(data).await {
                                    Err(why) => {
                                        msg = "Create response error, sorry for inconvenience".to_string();
                                        eprintln!("Create response error: {:?}", why)
                                    },
                                    Ok(response_msg) => {
                                        msg = response_msg;
                                    }
                                }
                            }

                            match api.send(SendMessage::new(message.chat, msg)).await {
                                Err(why) => {
                                    eprintln!("Send message error: {:?}", why)
                                },
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

async fn create_response(data: &str) -> Result<String> {
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

    let mut value ="---- Standard Error ----\n\n".to_string();
    value.push_str(playground_response.stderr.as_str());
    if playground_response.success {
        value.push_str("\n---- Standard Output ----\n\n");
        value.push_str(playground_response.stdout.as_str());
    }

    Ok(value)
}
