use actix_web::Responder;
use crate::model;

struct Person {
    name: String,
    age: u32,
}

pub fn index() -> impl Responder {
    let p = Person {
        name: "hello".to_string(),
        age: 20,
    };
    model::APIResult::new(200, p, "success".to_string())
}
