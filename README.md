# Image API (beta) 

[![Build](https://github.com/arjunkomath/rust-image-api/actions/workflows/build.yml/badge.svg)](https://github.com/arjunkomath/rust-image-api/actions/workflows/build.yml)

A fast image manipulation API that can modify images on the fly.
This project is in an early stage and is in beta, this hasn't been production tested yet.

Example: https://rust-image-api.fly.dev/v1/resize/w/240?url=https://push.techulus.com/images/logo.png

[![Deploy on Railway](https://railway.app/button.svg)](https://railway.app/template/zHlq1G?referralCode=rXEVYY)

## Usage

#### `GET /v1/resize/w/<max-width>?url=<image-url>`
resize and serve the image from `<image-url>` to `<max-width>` pixels wide, without changing the aspect ratio

#### `GET /v1/resize/h/<max-height>?url=<image-url>`
resize and serve the image from `<image-url>` to `<max-height>` pixels tall, without changing the aspect ratio
      
#### `GET /v1/convert/<format>?url=<image-url>`
convert and serve the image from `<image-url>` to `<format>` format
format: png, jpeg, webp

#### `GET /v1/flip/<orientation>?url=<image-url>`
flip image from `<image-url>` to `<orientation>` orientation
orientation: horizontal, vertical

#### `GET /v1/grayscale?url=<image-url>`
convert image from `<image-url>` to grayscale

#### `GET /v1/invert?url=<image-url>`
invert image from `<image-url>`

#### `GET /v1/brighten/<value>?url=<image-url>`
brighten image from `<image-url>` by `<value>`, negative values decrease the brightness and positive values increase it

#### `GET /v1/blur/<sigma>?url=<image-url>`
blur image from `<image-url>` with `<sigma>` sigma (this is a slow endpoint and could potentially timeout)

#### `GET /v1/rotate/<deg>?url=<image-url>`
rotate image from `<image-url>` by `<deg>` degrees, degree can be 90, 180, 270

## Development

Run `cargo make watch`

## Deployment

This project can be deployed using the provided Dockerfile
