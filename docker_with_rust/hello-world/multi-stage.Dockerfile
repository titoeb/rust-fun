FROM rust:alpine3.14 as BUILDER
WORKDIR /usr/src/hello-world
COPY src/ ./src
COPY Cargo.toml ./
RUN cargo build --release

FROM alpine:3.14.2 as RUNNER
COPY --from=BUILDER /usr/src/hello-world/target/release/hello-world /app/run
CMD /app/run