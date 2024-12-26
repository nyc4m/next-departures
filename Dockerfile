FROM rust:1.79-slim-bullseye as base

FROM base as builder

RUN echo Y | apt-get update && echo Y | apt-get upgrade
RUN echo Y | apt-get install pkg-config libssl-dev

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:bullseye-slim

WORKDIR /app
RUN apt-get update -y
RUN apt-get install -y ca-certificates

COPY --from=builder /app/target/release/sncf-board ./sncf-board


EXPOSE 8080

ENTRYPOINT ["./sncf-board"]
