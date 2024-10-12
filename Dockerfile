FROM rust:latest AS builder

RUN cargo install sccache
ENV RUSTC_WRAPPER=sccache

WORKDIR /usr/src/easy_list
COPY Cargo.toml Secrets.toml ./
COPY src ./src
RUN cargo build --release

FROM debian:12-slim

RUN apt-get update && apt-get install -y openssl ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /usr/src/easy_list/Secrets.toml /app
COPY --from=builder /usr/src/easy_list/target/release/easy_list /usr/local/bin/easy_list

CMD ["/usr/local/bin/easy_list"]
