FROM rust:1.88-slim-bookworm AS builder

WORKDIR /app

COPY backend/Cargo.toml backend/Cargo.lock ./
COPY backend/src ./src
COPY postgres/initdb/001_schema.sql ../postgres/initdb/001_schema.sql

RUN cargo build --release --bin web --no-default-features --features web

FROM debian:bookworm-slim AS runtime

WORKDIR /app

COPY --from=builder /app/target/release/web /usr/local/bin/jazzdomizer-web

ENV APP_HOST=0.0.0.0
ENV APP_PORT=3000

EXPOSE 3000

CMD ["jazzdomizer-web"]
