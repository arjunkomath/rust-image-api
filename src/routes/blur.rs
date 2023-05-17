use crate::utils::http::{ImageHelper, ImageSource};
use actix_web::{get, web, Responder};

#[get("/blur/{sigma}")]
pub async fn blur_image(sigma: web::Path<u32>, query: web::Query<ImageSource>) -> impl Responder {
    let sigma = sigma.into_inner();

    if let Ok(image) = crate::utils::http::get_image_from_url(&query.url).await {
        let image = image.blur(sigma as f32);

        if let Ok(result) = ImageHelper::new(image).png_response() {
            return result;
        }
    }

    crate::utils::http::empty_response()
}
