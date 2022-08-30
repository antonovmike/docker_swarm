FROM rust

WORKDIR /app

COPY . .

CMD ["rust", "src/main.rs"]
