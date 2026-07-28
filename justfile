image_name := "image-api"
tag := "latest"

build:
    cargo build

run:
    cargo run

test:
    cargo test

clippy:
    cargo clippy

check:
    cargo check

clean:
    cargo clean

docker-build:
    docker build -t {{ image_name }}:{{ tag }} .

docker-run: docker-build
    docker run --rm -p 8080:8080 {{ image_name }}:{{ tag }}
