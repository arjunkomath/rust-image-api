use crate::utils::http::{EmptyResponse, ImagePayload, ImageResponse};
use actix_web::{get, web, HttpResponse, Result};

#[get("/rotate/{deg}")]
pub async fn rotate(
    deg: web::Path<String>,
    payload: ImagePayload,
) -> Result<HttpResponse, EmptyResponse> {
    let deg = deg.into_inner();

    let image = match deg.as_str() {
        "90" => payload.image.rotate90(),
        "180" => payload.image.rotate180(),
        "270" => payload.image.rotate270(),
        _ => payload.image,
    };

    ImageResponse {
        data: image,
        format: image::ImageFormat::Png,
    }
    .try_into()
}
