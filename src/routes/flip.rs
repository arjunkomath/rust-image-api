use crate::utils::http::{EmptyResponse, ImagePayload, ImageResponse};
use actix_web::{get, web, HttpResponse, Result};

#[get("/flip/{orientation}")]
pub async fn flip_orientation(
    orientation: web::Path<String>,
    payload: ImagePayload,
) -> Result<HttpResponse, EmptyResponse> {
    let orientation = orientation.into_inner();

    let image = match orientation.as_str() {
        "horizontal" => payload.image.fliph(),
        "vertical" => payload.image.flipv(),
        _ => payload.image,
    };

    ImageResponse {
        data: image,
        format: image::ImageFormat::Png,
    }
    .try_into()
}
