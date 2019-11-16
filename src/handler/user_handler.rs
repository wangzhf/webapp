use actix_web::{web, Responder};
use serde::{Deserialize, Serialize};

use crate::model::APIResult;

#[derive(Serialize, Deserialize, Clone)]
pub struct Person {
    pub name: String,
    pub age: u32,
}

pub fn index() -> impl Responder {
    let p = Person {
        name: "hello".to_string(),
        age: 20,
    };
    APIResult::new(200, Some(p), None::<String>)
}

pub fn add(json: web::Json<Person>) -> impl Responder {
    let p = json.into_inner();
    APIResult::new(200, Some(p), None::<String>)
}
