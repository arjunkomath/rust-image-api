use crate::utils::http::{get_image_from_url, image_response, BytesResponse};
use image::GenericImageView;

#[get("/w/<width>?<url>")]
pub async fn resize_by_width(width: u32, url: &str) -> BytesResponse {
    if let Ok(image) = get_image_from_url(&url).await {
        // Get the original dimensions
        let (original_width, original_height) = image.dimensions();

        // Calculate the new height while maintaining the aspect ratio
        let new_height = (width as f32 * original_height as f32 / original_width as f32) as u32;

        // Resize the image
        let resized_image =
            image.resize_exact(width, new_height, image::imageops::FilterType::Lanczos3);

        if let Ok(result) = image_response(resized_image) {
            return result;
        }
    }

    BytesResponse::new(Vec::new()) // Return an empty byte vector
}

#[get("/h/<height>?<url>")]
pub async fn resize_by_height(height: u32, url: &str) -> BytesResponse {
    if let Ok(image) = get_image_from_url(&url).await {
        // Get the original dimensions
        let (original_width, original_height) = image.dimensions();

        // Calculate the new height while maintaining the aspect ratio
        let new_width = (height as f32 * original_width as f32 / original_height as f32) as u32;

        // Resize the image
        let resized_image =
            image.resize_exact(new_width, height, image::imageops::FilterType::Lanczos3);

        if let Ok(result) = image_response(resized_image) {
            return result;
        }
    }

    // Handle the case when the image download fails
    // For example, you could return a default image or an error message
    BytesResponse::new(Vec::new()) // Return an empty byte vector
}
