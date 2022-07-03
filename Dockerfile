FROM rust:1.62.0 as builder
WORKDIR /usr/src/starboard

# force cargo to update the crates.io index.
# putting this first means that the cache won't be
# invalidated if src/migrations/cargo.toml/cargo.lock
# is changed.
RUN cargo search --limit 0

# copy stuff over to the image that we need
COPY ./src ./src
COPY ./migrations ./migrations
COPY Cargo.toml Cargo.toml
COPY sqlx-data.json sqlx-data.json

# install starboard
RUN cargo install --path .

# get rid of cargo
FROM debian:buster-slim

# install certificates
RUN apt-get update && apt-get install -y ca-certificates

# copy starboard over from the builder
COPY --from=builder /usr/local/cargo/bin/starboard /usr/local/bin/starboard

# run starboard
CMD ["starboard"]
