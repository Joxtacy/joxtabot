FROM rust:1.76

WORKDIR /app

RUN cargo install cargo-watch

COPY . .

RUN cargo build

CMD ["cargo", "watch", "-x", "run", "-w", "./src", "-w", "Cargo.toml"]
