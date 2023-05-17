use crate::utils::http::{ImageHelper, ImageSource};
use actix_web::{get, web, Responder};
use image::GenericImageView;

#[get("/w/{width}")]
pub async fn resize_by_width(
    width: web::Path<u32>,
    query: web::Query<ImageSource>,
) -> impl Responder {
    let image_width = width.into_inner();

    if let Ok(image) = crate::utils::http::get_image_from_url(&query.url).await {
        // Get the original dimensions
        let (original_width, original_height) = image.dimensions();

        // Calculate the new height while maintaining the aspect ratio
        let new_height =
            (image_width as f32 * original_height as f32 / original_width as f32) as u32;

        // Resize the image
        let resized_image = image.resize_exact(
            image_width,
            new_height,
            image::imageops::FilterType::Triangle,
        );

        if let Ok(result) = ImageHelper::new(resized_image).png_response() {
            return result;
        }
    }

    crate::utils::http::empty_response()
}

#[get("/h/{height}")]
pub async fn resize_by_height(
    height: web::Path<u32>,
    query: web::Query<ImageSource>,
) -> impl Responder {
    let image_height = height.into_inner();

    if let Ok(image) = crate::utils::http::get_image_from_url(&query.url).await {
        // Get the original dimensions
        let (original_width, original_height) = image.dimensions();

        // Calculate the new height while maintaining the aspect ratio
        let new_width =
            (image_height as f32 * original_width as f32 / original_height as f32) as u32;

        // Resize the image
        let resized_image = image.resize_exact(
            new_width,
            image_height,
            image::imageops::FilterType::Triangle,
        );

        if let Ok(result) = ImageHelper::new(resized_image).png_response() {
            return result;
        }
    }

    crate::utils::http::empty_response()
}
