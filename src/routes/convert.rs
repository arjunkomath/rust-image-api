use crate::utils::http::{EmptyResponse, ImageResponse, ImageSource};
use actix_web::{get, web, HttpResponse, Result};

#[get("/convert/{format}")]
pub async fn convert_type(
    format: web::Path<String>,
    query: web::Query<ImageSource>,
) -> Result<HttpResponse, EmptyResponse> {
    let format = format.into_inner();

    let image = crate::utils::http::get_image_from_url(&query.url).await?;

    match format.as_str() {
        "png" => ImageResponse {
            data: image,
            format: image::ImageFormat::Png,
        },
        "jpeg" => ImageResponse {
            data: image,
            format: image::ImageFormat::Jpeg,
        },
        "webp" => ImageResponse {
            data: image,
            format: image::ImageFormat::WebP,
        },
        _ => ImageResponse {
            data: image,
            format: image::ImageFormat::Png,
        },
    }
    .try_into()
}
