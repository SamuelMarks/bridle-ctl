FROM rust:nightly AS builder
RUN apt-get update && apt-get install -y libsqlite3-dev libpq-dev cmake build-essential && rm -rf /var/lib/apt/lists/*
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libsqlite3-0 libpq5 ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /usr/src/app/target/release/bridle-cli /usr/local/bin/
COPY --from=builder /usr/src/app/target/release/bridle-rest /usr/local/bin/
COPY --from=builder /usr/src/app/target/release/bridle-rpc /usr/local/bin/
COPY --from=builder /usr/src/app/target/release/bridle-agent /usr/local/bin/
CMD ["bridle-cli"]
