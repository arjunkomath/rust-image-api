# ---------------------------------------------------
# 1 - Build Stage
# ---------------------------------------------------
FROM rust:1.93.1 AS build

WORKDIR /usr/src/image-api
COPY . .

RUN cargo install --path .

# ---------------------------------------------------
# 2 - Deploy Stage
# ---------------------------------------------------
FROM debian:trixie-slim

RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Application files
COPY --from=build /usr/local/cargo/bin/image-api /usr/local/bin/image-api

EXPOSE 8080

CMD ["image-api"]
