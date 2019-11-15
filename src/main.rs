use actix_web::{web, App, HttpServer, Responder, HttpResponse, guard};

mod handler;
mod model;

// global data
struct AppState {
    app_name: String,
}

fn main() {

    HttpServer::new(|| {
        App::new()
            .data(AppState {
                app_name: "Webapp".to_string()
            })
            .service(web::scope("/app")
                .route("/", web::get().to(handler::index))
            )

            .default_service(
                web::resource("")
                    .route(web::get().to(h404))
                    // all requests that are not `GET`
                    .route(
                        web::route()
                            .guard(guard::Not(guard::Get()))
                            .to(HttpResponse::MethodNotAllowed)
                    )
            )
    })
        .bind("127.0.0.1:8088")
        .unwrap()
        .run()
        .unwrap();
}

// 404 handler
fn h404() -> impl Responder {
    HttpResponse::NotFound().body("Not found")
}
