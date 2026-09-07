FROM rust:slim

WORKDIR /app

COPY . .

RUN cargo build --release

CMD ["./target/release/infra_vet"]
