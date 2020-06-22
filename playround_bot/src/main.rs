use std::env;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use async_std::task;
use futures::{
    future::FutureExt,
    pin_mut,
    select,
    StreamExt,
};
use telegram_bot::*;

use playround_bot::*;

#[tokio::main]
async fn main() {
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN is not set");
    let file_path = env::var("FILE_PATH").expect("FILE_PATH is not set");

    let users = load_users_data(&file_path);

    let signal_handler_task = signal_handler().fuse();
    let commands_handler_task = commands_handler(token, users.clone()).fuse();

    pin_mut!(signal_handler_task, commands_handler_task);

    select! {
        () = commands_handler_task => eprintln!("Commands handler completed"),
        () = signal_handler_task => eprintln!("Signal handler completed"),
    }

    save_users_data(&file_path, users.clone());
}

async fn signal_handler() {
    let term = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::SIGTERM, Arc::clone(&term)).expect("SIGTERM signal handler set error");
    signal_hook::flag::register(signal_hook::SIGINT, Arc::clone(&term)).expect("SIGINT signal handler set error");

    while !term.load(Ordering::Relaxed) {
        task::sleep(Duration::from_secs(1)).await;
    }
}

async fn commands_handler(token: String, users: Users) {
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
                                    get_info(user.id, users.clone())
                                } else if data.starts_with("/set_channel ") {
                                    set_channel(user.id, users.clone(), data.split("/set_channel ").collect())
                                } else if data.starts_with("/set_mode ") {
                                    set_mode(user.id, users.clone(), data.split("/set_mode ").collect())
                                } else if data.starts_with("/set_edition ") {
                                    set_edition(user.id, users.clone(), data.split("/set_edition ").collect())
                                } else if data.starts_with("/set_backtrace ") {
                                    set_backtrace(user.id, users.clone(), data.split("/set_backtrace ").collect())
                                } else if data.starts_with("/set_build_type ") {
                                    set_build_type(user.id, users.clone(), data.split("/set_build_type ").collect())
                                } else {
                                    match create_response(user.id, users.clone(), data).await {
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