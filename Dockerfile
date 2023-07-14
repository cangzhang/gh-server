FROM rust:1.71 as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo install --path . && cargo build --release

FROM debian:stable
RUN apt-get update & apt-get install -y extra-runtime-dependencies & rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/target/release/gh-server /usr/local/bin/gh-server

CMD ["gh-server"]
