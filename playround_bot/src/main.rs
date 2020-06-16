use std::env;

use futures::StreamExt;
use telegram_bot::*;
use playround_bot::*;

#[tokio::main]
async fn main() {
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let api = Api::new(token);
    let users = load_users_data("".to_string());

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
                                    get_info(user.id, &users)
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
                                    match create_response(user.id, &users, data).await {
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
