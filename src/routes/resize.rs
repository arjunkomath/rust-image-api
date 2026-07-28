use crate::utils::http::{ApiError, ImagePayload, ImageResponse};
use actix_web::{HttpResponse, Result, get, web};

#[get("/w/{width}")]
pub async fn resize_by_width(
    width: web::Path<u32>,
    payload: ImagePayload,
) -> Result<HttpResponse, ApiError> {
    let image_width = width.into_inner();
    let resized_image =
        payload
            .image
            .resize(image_width, u32::MAX, image::imageops::FilterType::Triangle);

    ImageResponse {
        data: resized_image,
        format: image::ImageFormat::Png,
    }
    .try_into()
}

#[get("/h/{height}")]
pub async fn resize_by_height(
    height: web::Path<u32>,
    payload: ImagePayload,
) -> Result<HttpResponse, ApiError> {
    let image_height = height.into_inner();
    let resized_image = payload.image.resize(
        u32::MAX,
        image_height,
        image::imageops::FilterType::Triangle,
    );

    ImageResponse {
        data: resized_image,
        format: image::ImageFormat::Png,
    }
    .try_into()
}
