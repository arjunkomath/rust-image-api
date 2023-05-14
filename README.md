# Image API

An API to resize images on the fly
 
## Usage

Endpoint: `GET /image/<max-width>?url=<image-url>`

It will resize and serve the image from `<image-url>` to `<max-width>` pixels wide

Example: https://rust-image-api-production.up.railway.app/image/240?url=https://push.techulus.com/images/logo.png