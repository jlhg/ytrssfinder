FROM rust:1.73.0 as build
WORKDIR /app
COPY ./server /app
RUN cargo install --locked --path .

FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=build /usr/local/cargo/bin/ytrssfinder-server /usr/local/bin/ytrssfinder-server
EXPOSE 6000
CMD ["ytrssfinder-server"]
