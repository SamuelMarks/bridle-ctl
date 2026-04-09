FROM rust:nightly-alpine AS builder
RUN apk add --no-cache musl-dev sqlite-dev postgresql-dev cmake make g++
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM alpine:latest
RUN apk add --no-cache libpq sqlite-libs ca-certificates
WORKDIR /app
COPY --from=builder /usr/src/app/target/release/bridle-cli /usr/local/bin/
COPY --from=builder /usr/src/app/target/release/bridle-rest /usr/local/bin/
COPY --from=builder /usr/src/app/target/release/bridle-rpc /usr/local/bin/
COPY --from=builder /usr/src/app/target/release/bridle-agent /usr/local/bin/
CMD ["bridle-cli"]
