FROM rust as build

WORKDIR /usr/src/
COPY . .

ARG GITHUB_TOKEN
RUN git config --global url."https://${GITHUB_TOKEN}@github.com/".insteadOf "https://github.com/"

RUN cargo build --release

FROM debian:buster-slim

COPY --from=build /usr/src/target/release/bot /app/bot
WORKDIR /app

EXPOSE 8000

RUN apt-get update && apt-get install libssl-dev ca-certificates -y && rm -rf /var/lib/apt/lists/*

ENTRYPOINT ["/app/bot"]
