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
                        api.send(message.text_reply(get_start_message().await)).await?;
                    } else {
                        api.send(message.text_reply(create_response(data).await)).await?;
                    }
                }
            }
        }
    }
}

async fn get_start_message() -> String {
    "Welcome! Let's go deeper to Rust. It's Rust Playground Bot. You can check some piece of your Rust code, sending it to me.".to_string()
}

#[derive(Serialize)]
pub struct PlaygroundRequest {
    code: String,
    version: String,
    optimize: String,
    test: bool,
    separate_output: bool,
    color: bool,
    backtrace: String,
}

async fn create_response(data: &str) -> String {
    let connector = HttpsConnector::new();
    let client = hyper::Client::builder().build(connector);

    let playground_request = serde_json::to_string(&PlaygroundRequest {
        code: data.to_string(),
        version: String::from("stable"),
        optimize: String::from("0"),
        test: false,
        separate_output: true,
        color: false,
        backtrace: String::from("0"),
    }).unwrap();

    let req = hyper::Request::builder()
        .method("POST")
        .uri("https://play.rust-lang.org/evaluate.json")
        .body(hyper::Body::from(playground_request))
        .expect("request builder");

    let body = client.request(req).await.unwrap();

    let bytes = hyper::body::to_bytes(body).await.unwrap();

    std::str::from_utf8(&bytes[..]).unwrap().to_string()
}
