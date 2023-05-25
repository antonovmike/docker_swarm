FROM rust

WORKDIR /app

COPY . .

RUN cargo build --release

ENTRYPOINT ["target/release/docker_swarm"]
