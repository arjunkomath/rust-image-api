use actix_web::{
    http::header::{CacheControl, CacheDirective, ContentType, ETag, EntityTag, IfNoneMatch},
    HttpResponse,
};
use image::DynamicImage;
use reqwest::Client;
use std::io::Cursor;

pub async fn get_image_from_url(url: &str) -> anyhow::Result<DynamicImage> {
    let client = Client::new();
    let response = client.get(url).send().await?;
    let bytes = response.bytes().await?;
    let image = image::load_from_memory(&bytes)?;

    Ok(image)
}

pub fn image_response(image: DynamicImage) -> anyhow::Result<HttpResponse> {
    let mut bytes = Vec::new();
    image.write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png)?;

    let etag_value = format!("{:x}", md5::compute(&bytes));

    Ok(HttpResponse::Ok()
        .content_type(ContentType::png())
        .insert_header(CacheControl(vec![CacheDirective::MaxAge(360u32)]))
        .insert_header(ETag(EntityTag::new_strong(etag_value.to_owned())))
        .insert_header(IfNoneMatch::Items(vec![EntityTag::new(
            false,
            etag_value.to_owned(),
        )]))
        .body(bytes))
}
