# Image API

[![Build](https://github.com/arjunkomath/rust-image-api/actions/workflows/build.yml/badge.svg)](https://github.com/arjunkomath/rust-image-api/actions/workflows/build.yml)

A fast image manipulation API that can modify images on the fly.
This project is in an early stage and is in beta, this hasn't been production tested yet.

Example: https://rust-image-api.fly.dev/v1/resize/w/240?url=https://push.techulus.com/images/logo.png

[![Deploy on Railway](https://railway.app/button.svg)](https://railway.app/template/zHlq1G?referralCode=rXEVYY)

## Usage

```
GET /v1/resize/w/<max-width>?url=<image-url>
resize image `<max-width>` pixels wide, without changing the aspect ratio

GET /v1/resize/h/<max-height>?url=<image-url>
resize image to `<max-height>` pixels tall, without changing the aspect ratio

GET /v1/convert/<format>?url=<image-url>
convert image to `<format>` format
format: png, jpeg, webp

GET /v1/flip/<orientation>?url=<image-url>
flip image  to `<orientation>` orientation
orientation: horizontal, vertical

GET /v1/grayscale?url=<image-url>
convert image to grayscale

GET /v1/invert?url=<image-url>
invert image

GET /v1/brighten/<value>?url=<image-url>
brighten image by `<value>`, negative values decrease the brightness and positive values increase it

GET /v1/blur/<sigma>?url=<image-url>
blur image with `<sigma>` sigma (this is a slow endpoint and could potentially timeout)

GET /v1/rotate/<deg>?url=<image-url>
rotate image by `<deg>` degrees, degree can be 90, 180, 270

GET /v1/unsharpen/<sigma>/<threshold>?url=<image-url>
unsharpen image, sigma is the amount to blur the image by, threshold is a control of how much to sharpen.
```

## Development

Run `cargo make watch`

## Deployment

This project can be deployed using the provided Dockerfile
