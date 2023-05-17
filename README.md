# Image API (beta)

A fast image manipulation API that can modify images on the fly.
This project is in an early stage and is in beta, this hasn't been production tested yet.

## Usage

### GET `/v1/resize/w/<max-width>?url=<image-url>`
resize and serve the image from `<image-url>` to `<max-width>` pixels wide, without changing the aspect ratio

### GET `/v1/resize/h/<max-height>?url=<image-url>`
resize and serve the image from `<image-url>` to `<max-height>` pixels tall, without changing the aspect ratio
      
### GET `/v1/convert/<format>?url=<image-url>`
convert and serve the image from `<image-url>` to `<format>` format
format: png, jpeg, webp

Example: https://rust-image-api-production.up.railway.app/v1/resize/w/240?url=https://push.techulus.com/images/logo.png

## Development

Run `cargo make watch`

## Deployment

This project can be deployed using the provided Dockerfile
