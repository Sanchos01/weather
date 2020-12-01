FROM rust:1.48 as builder
WORKDIR /usr/src/weather
COPY . .
RUN cargo build --release

FROM debian:buster-slim
RUN apt-get update && apt-get install -y openssl && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/weather/target/release/weather /usr/local/bin/weather
EXPOSE 8080
CMD ["weather"]