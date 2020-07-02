# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------

FROM rust:1.44.1 as cargo-build

RUN apt-get update

RUN apt-get upgrade -y

WORKDIR /usr/src/playground_bot/

COPY Cargo.toml Cargo.toml

RUN mkdir src/

RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

RUN cargo build --release

RUN rm -fr target/

COPY . .

RUN cargo build --release

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

FROM rust:1.44.1-slim

RUN useradd -ms /bin/bash playground

RUN mkdir -p /home/playground/bin/

WORKDIR /home/playground/bin/

COPY --from=cargo-build /usr/src/playground_bot/target/release/playground_bot .

RUN chown -R playground:playground /home/playground

USER playground

CMD ["./playground_bot"]