# Image API

A fast image manipulation API that can modify images on the fly.
This project is in an early stage and is in beta, this hasn't been production tested yet.

## Usage

### GET /image/w/<max-width>?url=<image-url>
resize and serve the image from `<image-url>` to `<max-width>` pixels wide, without changing the aspect ratio

### GET /image/h/<max-height>?url=<image-url>
resize and serve the image from `<image-url>` to `<max-height>` pixels tall, without changing the aspect ratio
      
### GET /image/convert/<format>?url=<image-url>
convert and serve the image from `<image-url>` to `<format>` format
format: png, jpeg, webp

Example: https://rust-image-api-production.up.railway.app/image/w/240?url=https://push.techulus.com/images/logo.png

## Development

Run `cargo make watch`

## Deployment

This project can be deployed using the provided Dockerfile