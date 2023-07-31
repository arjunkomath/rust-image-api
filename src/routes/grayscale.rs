use crate::utils::http::{EmptyResponse, ImageResponse, ImageSource};
use actix_web::{get, web, HttpResponse};
use image::ImageFormat;

#[get("/grayscale")]
pub async fn grayscale(query: web::Query<ImageSource>) -> Result<HttpResponse, EmptyResponse> {
    let image = crate::utils::http::get_image_from_url(&query.url)
        .await
        .map_err(|_| EmptyResponse {})?;

    let image = image.grayscale();

    ImageResponse {
        data: image,
        format: ImageFormat::Png,
    }
    .try_into()
}
