FROM rust

WORKDIR /app

COPY . .

CMD ["rust", "src/main.rs"]

# RUN ["./docker_swarm"]
