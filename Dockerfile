FROM rust:1.70.0-slim-buster as builder
WORKDIR /usr/src/starboard

# nightly
RUN rustup default nightly

# force cargo to update the crates.io index.
RUN cargo search --limit 0

# install pkg-config (required for reqwest dependency)
RUN apt-get update
RUN apt-get install -y pkg-config libssl-dev

# # cache dependencies
COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml

COPY crates/common/Cargo.toml crates/common/Cargo.toml
COPY crates/database/Cargo.toml crates/database/Cargo.toml
COPY crates/errors/Cargo.toml crates/errors/Cargo.toml
COPY crates/starboard/Cargo.toml crates/starboard/Cargo.toml

RUN mkdir crates/common/src && touch crates/common/src/lib.rs
RUN mkdir crates/database/src && touch crates/database/src/lib.rs
RUN mkdir crates/errors/src && touch crates/errors/src/lib.rs
RUN mkdir crates/starboard/src && echo "fn main() { dbg!(1); }" > crates/starboard/src/main.rs

RUN cargo build --release
RUN rm target/release/starboard
RUN rm -r crates

# copy what we need
COPY crates crates
COPY sqlx-data.json sqlx-data.json

# install starboard
RUN cargo install cargo-edit
RUN cargo set-version --workspace --bump major
RUN cargo build --release

# get rid of cargo
FROM debian:buster-slim

# install certificates
RUN apt-get update && apt-get install -y ca-certificates

# copy starboard over from the builder
COPY --from=builder /usr/src/starboard/target/release/starboard /usr/local/bin/starboard

# run starboard
CMD ["starboard"]
