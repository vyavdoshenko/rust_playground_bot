use std::env;

use futures::StreamExt;
use hyper_rustls::HttpsConnector;
use serde::Serialize;
use telegram_bot::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let api = Api::new(token);

    // Fetch new updates via long poll method
    let mut stream = api.stream();
    loop {
        if let Some(update) = stream.next().await {
            // If the received update contains a new message...
            let update = update?;
            if let UpdateKind::Message(message) = update.kind {
                if let MessageKind::Text { ref data, .. } = message.kind {
                    if data == "/start" {
                        api.send(message.text_reply(get_start_message())).await?;
                    } else {
                        api.send(message.text_reply(create_response(data).await?)).await?;
                    }
                }
            }
        }
    }
}

fn get_start_message() -> String {
    "Welcome! Let's go deeper to Rust. It's Rust Playground Bot. You can check some piece of your Rust code, sending it to me.".to_string()
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

    let req = hyper::Request::post("https://play.rust-lang.org/execute")//("http://localhost:3000/test")
        .header("content-type", "application/x-www-form-urlencoded")//("https://play.rust-lang.org/execute")
        .body(hyper::Body::from(playground_request))?;

    let body = client.request(req).await?;
    let bytes = hyper::body::to_bytes(body).await?;

    return Ok(std::str::from_utf8(&bytes[..])?.to_string())
}
