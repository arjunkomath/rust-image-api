use crate::utils::http::{EmptyResponse, ImageResponse, ImageSource};
use actix_web::{get, web, HttpResponse, Result};

#[get("/blur/{sigma}")]
pub async fn blur_image(
    sigma: web::Path<u32>,
    query: web::Query<ImageSource>,
) -> Result<HttpResponse, EmptyResponse> {
    let sigma = sigma.into_inner();

    let image = crate::utils::http::get_image_from_url(&query.url)
        .await
        .map_err(|_| EmptyResponse {})?;

    let image = image.blur(sigma as f32);

    ImageResponse {
        data: image,
        format: image::ImageFormat::Png,
    }
    .try_into()
}
