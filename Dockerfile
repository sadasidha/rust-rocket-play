# docker build -t robota-aggregate .
FROM rust:1.84-bullseye AS builder
WORKDIR /appsrc/
COPY . .
RUN cargo build --release
# running environment
FROM debian:12.5-slim
RUN apt-get update
RUN apt-get install -y curl
COPY --from=builder /appsrc/target/release/robota-aggregate /app
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=7128
CMD [ "/app" ]
EXPOSE $ROCKET_PORT
