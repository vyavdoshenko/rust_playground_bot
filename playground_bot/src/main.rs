use std::env;

use futures::{future::FutureExt, pin_mut, select};

use playground_bot::*;

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
