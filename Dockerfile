FROM rust:1.71 as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo install --path . && cargo build --release --bin app

FROM debian:stable
RUN apt-get update & apt-get install -y extra-runtime-dependencies & rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/target/release/app /usr/local/bin/app

CMD ["app"]
