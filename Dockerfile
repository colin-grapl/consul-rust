FROM rust:1.55 AS base
SHELL ["/bin/bash", "-c"]

ENV HOME=/home/root

WORKDIR /tmp

WORKDIR $HOME/consul-rust/
RUN mkdir ./.cargo/ && touch ./.cargo/config

COPY ./src/ ./src/
COPY ./tests/ ./tests/
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock


RUN --mount=type=cache,target=/root/home/.cache \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=target/ \
    --mount=type=cache,target=/root/home/.cargo \
      cargo vendor > .cargo/config

RUN --mount=type=cache,target=/root/home/.cache \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=target/ \
    --mount=type=cache,target=/root/home/.cargo \
      cargo test --no-run

ENTRYPOINT ["cargo", "test", "--", "--nocapture"]
