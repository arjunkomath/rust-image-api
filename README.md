# Image API

An API to resize images on the fly
 
## Usage

### Resize by Width

`GET /image/w/<max-width>?url=<image-url>`

resize and serve the image from `<image-url>` to `<max-width>` pixels wide, without changing the aspect ratio

Example: https://rust-image-api-production.up.railway.app/image/w/240?url=https://push.techulus.com/images/logo.png
        
### Resize by Height

`GET /image/h/<max-height>?url=<image-url>`

resize and serve the image from `<image-url>` to `<max-height>` pixels tall, without changing the aspect ratio

Example: https://rust-image-api-production.up.railway.app/image/h/640?url=https://push.techulus.com/images/logo.png

## Development

Run `cargo make watch`