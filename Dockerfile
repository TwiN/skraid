# Build the go application into a binary
FROM ekidd/rust-musl-builder:latest as builder
WORKDIR /app
ADD --chown=rust:rust . ./
RUN cargo build --release

FROM alpine:latest
ENV APP_HOME=/app
ENV DISCORD_BOT_TOKEN=""
ENV MAINTAINER_ID=""
ENV COMMAND_PREFIX="s!"
ENV DATABASE_PATH=""
WORKDIR ${APP_HOME}
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/skraid /app/bin/
ENTRYPOINT ["/app/bin/skraid"]