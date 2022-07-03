FROM rust:1.62.0 as builder
WORKDIR /usr/src/starboard
COPY ./src ./src
COPY ./migrations ./migrations
COPY Cargo.toml Cargo.toml
COPY sqlx-data.json sqlx-data.json
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y ca-certificates
COPY --from=builder /usr/local/cargo/bin/starboard /usr/local/bin/starboard
CMD ["starboard"]
