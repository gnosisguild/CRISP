FROM lukemathwalker/cargo-chef:latest as chef
WORKDIR /app

FROM chef AS planner
COPY ./Cargo.toml ./Cargo.lock ./
COPY ./src ./src
RUN cargo chef prepare

FROM chef AS builder
COPY --from=planner /app/recipe.json .
RUN cargo chef cook --release
COPY . .
RUN --mount=type=secret,id=INFURAKEY \
  export INFURAKEY=$(cat /run/secrets/INFURAKEY) && \
  cargo build --release
CMD ["./target/release/start_cipher_node"]