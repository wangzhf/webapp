use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct APIResult<T> {
    code: u32,
    data: Box<Option<T>>,
    message: String,
}

impl<T> APIResult<T> {
    pub fn new(code: u32, data: Box<Option<T>>, message: String) -> APIResult<T> {
        APIResult {
            code,
            data,
            message,
        }
    }
}

impl<T> Responder for APIResult<T>
where
    T: Serialize,
{
    type Error = Error;
    type Future = Result<HttpResponse, Error>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self)?;

        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body))
    }
}
