use std::env;
use futures::StreamExt;
use telegram_bot::*;
use playround_bot::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN is not set");
    let file_path = env::var("FILE_PATH").expect("FILE_PATH is not set");
    let api = Api::new(token);
    let mut users = load_users_data(&file_path);

    let term = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::SIGTERM, Arc::clone(&term)).expect("Signal handler set error");

    // Fetch new updates via long poll method
    let mut stream = api.stream();
    while !term.load(Ordering::Relaxed) {
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
                                } else if data.starts_with("/set_channel ") {
                                    set_channel(user.id, &mut users, data.split("/set_channel ").collect())
                                } else if data.starts_with("/set_mode ") {
                                    set_mode(user.id, &mut users, data.split("/set_mode ").collect())
                                } else if data.starts_with("/set_edition ") {
                                    set_edition(user.id, &mut users, data.split("/set_edition ").collect())
                                } else if data.starts_with("/set_backtrace ") {
                                    set_backtrace(user.id, &mut users, data.split("/set_backtrace ").collect())
                                } else if data.starts_with("/set_build_type ") {
                                    set_build_type(user.id, &mut users, data.split("/set_build_type ").collect())
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

    save_users_data(&file_path, &users);
}
