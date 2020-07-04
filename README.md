## Telegram bot for using Rust playground.

You can check some pieces of your Rust code, sending it to the Telegram Bot.
It will check the code using Rust playground: https://play.rust-lang.org/

* Telegram Bots info: https://core.telegram.org/bots

* Create and tune bots: https://telegram.me/BotFather

### Supported bot's commands:

Welcome message.
```
/start
```

Get link to Rust playground service.
```
/playground
```

Get link to this repository.
```
/github
```

Set channel. Default is stable.
```
/set_channel stable|beta|nightly
```

Set mode. Default is debug.
```
/set_mode debug|release
```

Set edition. Default is 2018.
```
/set_edition 2018|2015
```

Set backtrace. Default is disabled.
```
/set_backtrace disabled|enabled
```

What do you want to do? Default is run.
```
/set_build_type run|build|test
```

Get settings information.
```
/info
```

#### Info from Rust playground:

Build and run the code, showing the output. Equivalent to cargo run.
```
/set_build_type run
```

Build the code without running it. Equivalent to cargo build.
```
/set_build_type build
```

Build the code and run all the tests. Equivalent to cargo test.
```
/set_build_type test
```

## Docker build & run:

Build the docker container.
```
$ cd playground_bot
$ docker build -t playground_bot .
```

Run the application.
```
$ docker run -d --env TELEGRAM_BOT_TOKEN=1234567:YOUR_BOT_TOKEN --rm -it playground_bot
```
Take attention that 1234567:YOUR_BOT_TOKEN should be replaced with your bot ID. It should be gotten from Telegram BotFather.

## TODO:

The bot can process only execute commands from following request: https://play.rust-lang.org/execute
run/build/test

I plan to add compile commands in future if it needed: https://play.rust-lang.org/compile
Please contact me to make request this feature.
ASM/LLVM IR/MIR/WASM

## Contribution:
Feel free to create pull request or contact me to discuss for add/modify code or something else via GitHub or email: v.yavdoshenko at gmail.com

## Live bot:
You can try the latest version of this bot here:
https://telegram.me/rust_playground_bot

## DockerHub:
https://hub.docker.com/r/yavdoshenko/rust_playground_bot