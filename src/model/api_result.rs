use actix_web::{Responder, HttpRequest, Error, HttpResponse};
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize)]
pub struct APIResult<T> {
    code: u32,
    data: T,
    message: String,
}

impl<T> APIResult<T> where T: Display {

    pub fn new<T>(code: u32, data: T, message: String) -> APIResult<T> {
        APIResult {
            code,
            data,
            message,
        }
    }

}

impl<T> Responder for APIResult<T> where T: Display {
    type Error = Error;
    type Future = Result<HttpResponse, Error>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {

        let body = serde_json::to_string(&self)?;

        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)
        )
    }
}

impl<T> Display for APIResult<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {

    }
}
