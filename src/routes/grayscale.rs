use crate::utils::http::{auto_image_format, EmptyResponse, ImagePayload, ImageResponse};
use actix_web::{get, HttpRequest, HttpResponse};

#[get("/grayscale")]
pub async fn handler(
    req: HttpRequest,
    payload: ImagePayload,
) -> Result<HttpResponse, EmptyResponse> {
    ImageResponse {
        data: payload.image.grayscale(),
        format: auto_image_format(&req),
    }
    .try_into()
}

#[cfg(test)]
mod tests {
    use image::{DynamicImage, Rgb, RgbImage};

    #[test]
    fn test_grayscale_converts_colored_image() {
        let mut img = RgbImage::new(2, 2);
        img.put_pixel(0, 0, Rgb([255, 0, 0]));
        img.put_pixel(1, 0, Rgb([0, 255, 0]));
        img.put_pixel(0, 1, Rgb([0, 0, 255]));
        img.put_pixel(1, 1, Rgb([255, 255, 0]));

        let dynamic_img = DynamicImage::ImageRgb8(img);
        let grayscale_img = dynamic_img.grayscale();
        let luma_img = grayscale_img.to_luma8();

        assert_eq!(luma_img.get_pixel(0, 0)[0], 54);
        assert_eq!(luma_img.get_pixel(1, 0)[0], 182);
        assert_eq!(luma_img.get_pixel(0, 1)[0], 18);
        assert_eq!(luma_img.get_pixel(1, 1)[0], 236);
    }

    #[test]
    fn test_grayscale_preserves_dimensions() {
        let img = RgbImage::new(100, 50);
        let dynamic_img = DynamicImage::ImageRgb8(img);
        let grayscale_img = dynamic_img.grayscale();

        assert_eq!(grayscale_img.width(), 100);
        assert_eq!(grayscale_img.height(), 50);
    }
}
