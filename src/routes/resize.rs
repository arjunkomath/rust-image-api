use crate::utils::http::{EmptyResponse, ImageResponse, ImageSource};
use actix_web::{get, web, HttpResponse, Result};
use image::GenericImageView;

#[get("/w/{width}")]
pub async fn resize_by_width(
    width: web::Path<u32>,
    query: web::Query<ImageSource>,
) -> Result<HttpResponse, EmptyResponse> {
    let image_width = width.into_inner();

    let image = crate::utils::http::get_image_from_url(&query.url).await?;

    // Get the original dimensions
    let (original_width, original_height) = image.dimensions();

    // Calculate the new height while maintaining the aspect ratio
    let new_height = (image_width as f32 * original_height as f32 / original_width as f32) as u32;

    // Resize the image
    let resized_image = image.resize_exact(
        image_width,
        new_height,
        image::imageops::FilterType::Triangle,
    );

    ImageResponse {
        data: resized_image,
        format: image::ImageFormat::Png,
    }
    .try_into()
}

#[get("/h/{height}")]
pub async fn resize_by_height(
    height: web::Path<u32>,
    query: web::Query<ImageSource>,
) -> Result<HttpResponse, EmptyResponse> {
    let image_height = height.into_inner();

    let image = crate::utils::http::get_image_from_url(&query.url).await?;

    // Get the original dimensions
    let (original_width, original_height) = image.dimensions();

    // Calculate the new height while maintaining the aspect ratio
    let new_width = (image_height as f32 * original_width as f32 / original_height as f32) as u32;

    // Resize the image
    let resized_image = image.resize_exact(
        new_width,
        image_height,
        image::imageops::FilterType::Triangle,
    );

    ImageResponse {
        data: resized_image,
        format: image::ImageFormat::Png,
    }
    .try_into()
}
