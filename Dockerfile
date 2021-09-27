FROM rust:1.55.0 as builder
WORKDIR /api
COPY . .
RUN cargo install --path .

FROM rust:1.55.0 as test
WORKDIR /api
COPY . .
RUN cargo test --tests

FROM debian:buster-slim
RUN apt-get update && apt-get install -y libpq-dev
COPY --from=builder /usr/local/cargo/bin/api /usr/local/bin/api
CMD ["api"]