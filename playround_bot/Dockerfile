 
FROM rust:latest

WORKDIR /usr/src/rust_playground_bot

COPY . .

SHELL ["/bin/bash", "-c", "cargo build --release"] 

CMD ["rust_playground_bot"]
