IMAGE_NAME := image-api
TAG := latest

.PHONY: build run test clippy check clean docker-build docker-run

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
	docker build -t $(IMAGE_NAME):$(TAG) .

docker-run: docker-build
	docker run --rm -p 8080:8080 $(IMAGE_NAME):$(TAG)
