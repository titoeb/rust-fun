FROM rust:alpine3.14
WORKDIR /usr/src/hello-world
COPY src/ ./src
COPY Cargo.toml ./
RUN cargo build --release
CMD cargo run