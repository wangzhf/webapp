use actix_web::{web, App, HttpServer};

mod handler;
mod model;

struct AppState {
    app_name: String,
}

fn main() {

    HttpServer::new(|| {
        App::new()
            .data(AppState {
                app_name: "Webapp".to_string()
            })
            .service(
            web::scope("/app")

                .route("/", web::get().to(handler::index))
        )
    })
        .bind("127.0.0.1:8088")
        .unwrap()
        .run()
        .unwrap();
}
