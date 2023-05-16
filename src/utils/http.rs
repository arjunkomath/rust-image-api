use std::io::Cursor;

use image::DynamicImage;
use reqwest::Client;
use rocket::{
    http::{ContentType, Status},
    response::{Responder, Response, Result},
    Request,
};

pub struct BytesResponse(Vec<u8>);

impl BytesResponse {
    pub fn new(data: Vec<u8>) -> Self {
        BytesResponse(data)
    }
}

impl<'r> Responder<'r, 'static> for BytesResponse {
    fn respond_to(self, req: &'r Request<'_>) -> Result<'static> {
        let bytes = self.0;

        let etag_value = format!("{:x}", md5::compute(&bytes));

        // Check if the request's ETag matches the current ETag
        if req.headers().contains("If-None-Match") {
            let if_none_match = req.headers().get("If-None-Match").next().unwrap_or("");
            if if_none_match == etag_value {
                return Response::build()
                    .status(Status::NotModified)
                    .raw_header("ETag", etag_value)
                    .ok();
            }
        }

        Response::build()
            .header(ContentType::PNG)
            .status(Status::Ok)
            .raw_header("Cache-Control", "public, max-age=31536000, immutable")
            .sized_body(bytes.len(), Cursor::new(bytes))
            .ok()
    }
}

pub async fn get_image_from_url(url: &str) -> anyhow::Result<DynamicImage> {
    let client = Client::new();
    let response = client.get(url).send().await?;
    let bytes = response.bytes().await?;
    let image = image::load_from_memory(&bytes)?;

    Ok(image)
}

pub fn image_response(image: DynamicImage) -> anyhow::Result<BytesResponse> {
    let mut bytes = Vec::new();
    image.write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png)?;

    Ok(BytesResponse::new(bytes))
}
