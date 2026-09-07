# ---------- Build ----------
FROM rust:slim AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./


RUN mkdir src \
  && echo "fn main {}" > src/main.rs \
  && cargo build --release

COPY src ./src

RUN cargo build --release


RUN chown -R appuser:appuser /app

USER appuser

# ---------- Runtime ----------
FROM debian:bookworm-slim

RUN useradd --create-home --shell /bin/bash appuser

WORKDIR /app

COPY --from=builder /app/target/release/app .

RUN chown appuser:appuser /app/app

USER appuser

CMD ["./app"]

