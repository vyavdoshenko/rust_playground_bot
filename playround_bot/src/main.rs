use std::env;

use futures::StreamExt;
use telegram_bot::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let api = Api::new(token);

    // Fetch new updates via long poll method
    let mut stream = api.stream();
    while let Some(update) = stream.next().await {
        // If the received update contains a new message...
        let update = update?;
        if let UpdateKind::Message(message) = update.kind {
            if let MessageKind::Text { ref data, .. } = message.kind {
                if data == "/start" {
                    api.send(message.text_reply(start_message())).await?;
                }

                api.send(message.text_reply(create_response(data))).await?;
            }
        }
    }
    Ok(())
}

fn start_message() -> String {
    "Welcome, and let's go deeper to Rust. It's Rust Playground Bot. You can check your little piece of Rust code, sending it to me.".to_string()
}

fn create_response(data: &str) -> String {
    format!("Hi! You just wrote '{}'", data)
}