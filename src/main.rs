use actix_web::{Responder, HttpResponse, HttpServer, App, web};

use actix_web::get;
use actix_web::guard;

fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello Rust!")
}

fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello Rust again!")
}

#[get("/hello")]
fn index3() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

struct AppState {
    app_name: String,
}

fn outState(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("Hello {}!", app_name)
}

fn main() {
    HttpServer::new(|| {
        App::new()

            //.route("/", web::get().to(index))
            //.route("/again", web::get().to(index2))
            .service(
                web::scope("/app")
                    .route("/index.html", web::get().to(index))
                    .route("/again.html", web::get().to(index2))
            )
            .service(index3)
            .default_service(
                web::route()
                    .guard(guard::Not(guard::Get()))
                    .to(|| HttpResponse::MethodNotAllowed())
            )

            .data(
                AppState {
                    app_name: String::from("Actix-web")
                }
            )
            .route("/", web::get().to(outState))
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}

