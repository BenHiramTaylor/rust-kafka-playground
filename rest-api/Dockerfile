FROM rust:1.79 AS builder

WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM debian:bullseye
RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/target/release/rest-api /usr/local/bin/rest-api

CMD ["rest-api"]