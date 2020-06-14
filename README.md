# Telegram bot for using Rust playground.

You can check some pieces of your Rust code, sending it to the Telegram Bot.
It will check the code using Rust playground: https://play.rust-lang.org/

* Telegram Bots info: https://core.telegram.org/bots

* Create and tune bots: https://telegram.me/BotFather

# Supported bot's commands:

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

Set version. Default is stable.
```
/version stable|beta|nightly
```

Set mode. Default is debug.
```
/mode debug|release
```

Set edition. Default is 2018.
```
/edition 2018|2015
```

Set backtrace. Default is disabled.
```
/backtrace disabled|enabled
```

What do you want to do? Default is run.
```
/cargo run|build|test|asm|llvm ir|mir|wasm
```

Get settings information.
```
/info
```

Info from Rust playground:

Build and run the code, showing the output. Equivalent to cargo run.
```
/cargo run
```

Build the code without running it. Equivalent to cargo build.
```
/cargo build
```

Build the code and run all the tests. Equivalent to cargo test.
```
/cargo test
```

Build and show the resulting assembly code.
```
/cargo asm
```

Build and show the resulting LLVM IR, LLVM’s intermediate representation.
```
/cargo llvm ir
```

Build and show the resulting MIR, Rust’s intermediate representation.
```
/cargo mir
```

Build a WebAssembly module for web browsers, in the .WAT textual representation.
Note: WASM currently requires using the Nightly channel, selecting this option will switch to Nightly.
```
/cargo wasm
```
