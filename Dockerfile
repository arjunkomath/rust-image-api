FROM rust:1.68.2

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8080

WORKDIR /app
COPY . .

RUN rustup default nightly
RUN cargo build

EXPOSE 8080

CMD ["cargo", "run"]