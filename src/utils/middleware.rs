use std::{
    future::{ready, Ready},
    rc::Rc,
};

use actix_web::{
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    web, Error, HttpMessage,
};
use futures_util::future::LocalBoxFuture;

use crate::utils::{self, http::ApiError};

use super::http::ImagePayload;

pub struct ImageParser;

impl<S: 'static, B> Transform<S, ServiceRequest> for ImageParser
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = ImageParserMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ImageParserMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct ImageParserMiddleware<S> {
    // This is special: We need this to avoid lifetime issues.
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for ImageParserMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();

        Box::pin(async move {
            let image_url = web::Query::<utils::http::ImageSource>::from_query(req.query_string());

            if let Ok(image_url) = image_url {
                let client =
                    req.app_data::<web::Data<reqwest::Client>>()
                        .ok_or(ApiError::InternalError(
                            "HTTP client not configured".to_string(),
                        ))?;
                let payload = ImagePayload::from_url(client, &image_url.url)
                    .await
                    .map_err(|e| ApiError::BadGateway(format!("Failed to fetch image: {e}")))?;

                req.extensions_mut().insert(payload);
            }

            let res = svc.call(req).await?;

            Ok(res)
        })
    }
}
