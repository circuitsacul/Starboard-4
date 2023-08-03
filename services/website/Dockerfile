FROM rust:1.71.0-slim-bullseye as prereq

RUN rustup default nightly
RUN rustup target add wasm32-unknown-unknown

RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash -
RUN apt-get update
RUN apt-get install -y pkg-config libssl-dev nodejs npm
RUN npm install -g sass

RUN cargo install cargo-chef
RUN cargo install cargo-leptos

FROM prereq AS planner
WORKDIR /app

COPY crates crates
COPY sqlx-data.json sqlx-data.json
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

RUN cargo chef prepare --recipe-path recipe.json

FROM prereq as cacher
WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --package=website --bin=website --target-dir=target/server --no-default-features --features=ssr  --recipe-path recipe.json
RUN cargo chef cook --release --package=website --target-dir=target/front --target=wasm32-unknown-unknown --no-default-features --features=hydrate  --recipe-path recipe.json

FROM prereq as builder
WORKDIR /app

COPY crates crates
COPY sqlx-data.json sqlx-data.json
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

# copy dependecies
COPY --from=cacher /app/target /app/target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
# set env variables for build
# The source style file. If it ends with _.sass_ or _.scss_ then it will be compiled by `dart-sass`
# into CSS and processed by lightning css. When release is set, then it will also be minified.
ENV LEPTOS_STYLE_FILE "style/main.scss"
# The browserlist https://browsersl.ist query used for optimizing the CSS.
ENV LEPTOS_BROWSERQUERY "defaults"
# build the app
RUN cargo leptos build --release

# use googles distroless as runtime image
FROM gcr.io/distroless/cc-debian11
# copy app form builder
COPY --from=builder /app/target/server/release/website /app/
COPY --from=builder /app/target/site /app/site
WORKDIR /app

# Site .env parameters cargo-leptos
ENV OUTPUT_NAME "website"
ENV LEPTOS_OUTPUT_NAME "website"
ENV LEPTOS_SITE_ROOT "site"
ENV LEPTOS_SITE_PKG_DIR "pkg"
ENV LEPTOS_ASSETS_DIR "assets"
ENV LEPTOS_SITE_ADDR "0.0.0.0:8000"

# start the application
CMD ["./website"]
