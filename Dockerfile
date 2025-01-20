
FROM rust:1-slim-bookworm AS builder
WORKDIR /code

RUN USER=root cargo init
COPY Cargo.toml Cargo.toml
RUN cargo fetch

COPY . .

RUN cargo build --release

FROM bitnami/minideb:bookworm

COPY --from=builder /code/target/release/xythrion-api /code/xythrion-api
COPY --from=builder /code/migrations/* /code/migrations/
WORKDIR /code

USER 1001
EXPOSE 8080

CMD [ "/code/xythrion-api" ]
