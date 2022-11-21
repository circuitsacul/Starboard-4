FROM rust:1.65.0-slim-buster as builder
WORKDIR /usr/src/starboard

# force cargo to update the crates.io index.
# putting this first means that the cache won't be
# invalidated if src/migrations/cargo.toml/cargo.lock
# is changed.
RUN cargo search --limit 0

# install pkg-config (required for reqwest dependency)
RUN apt-get update
RUN apt-get install -y pkg-config libssl-dev

# doing this allows caching of compiled dependencies
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
RUN mkdir src
RUN echo "fn main() { dbg!(1); }" > src/main.rs
RUN cargo build --release
RUN rm -r src && rm target/release/starboard

# copy stuff over to the image that we need
COPY build.rs build.rs
COPY ./src ./src
COPY ./migrations ./migrations
COPY sqlx-data.json sqlx-data.json

# install starboard
RUN cargo build --release

# get rid of cargo
FROM debian:buster-slim

# install certificates
RUN apt-get update && apt-get install -y ca-certificates

# copy starboard over from the builder
COPY --from=builder /usr/src/starboard/target/release/starboard /usr/local/bin/starboard

# run starboard
CMD ["starboard"]
