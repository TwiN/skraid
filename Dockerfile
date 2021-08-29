# Build the go application into a binary
FROM rust as builder
WORKDIR /app
ADD . ./
RUN cargo build --release

FROM scratch
ENV APP_HOME=/app
ENV DISCORD_BOT_TOKEN=""
ENV MAINTAINER_ID=""
ENV COMMAND_PREFIX="s!"
ENV DATABASE_PATH=""
WORKDIR ${APP_HOME}
COPY --from=builder /app/target/release/skraid ./bin/skraid
ENTRYPOINT ["/app/bin/skraid"]