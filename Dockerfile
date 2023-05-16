FROM rust:1.68.2

WORKDIR /app
COPY . .

RUN rustup default nightly
RUN cargo build

EXPOSE 8080

CMD ["cargo", "run"]