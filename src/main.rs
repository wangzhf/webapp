pub mod test;
mod httpserver;

use actix_web::{Responder, HttpResponse, HttpServer, App, web};

use actix_web::get;
use actix_web::guard;
use std::sync::Mutex;


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

fn out_state(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("Hello {}!", app_name)
}

fn init_server() {
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
            .route("/", web::get().to(out_state))
    })
        .bind("127.0.0.1:8088")
        .unwrap()
        .run()
        .unwrap();
}

// 定义应用间数据共享
struct AppStateWithCounter {
    counter: Mutex<i32>,
}

fn _index(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;

    format!("Request number: {}", counter)
}

fn init_share_data() {
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .register_data(counter.clone())
            .route("/", web::get().to(_index))
    })
        .bind("127.0.0.1:8088")
        .unwrap()
        .run()
        .unwrap();
}

fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/test")
            .route(web::get().to(|| {
                HttpResponse::Ok().body("test")
            }))
            .route(web::head().to(|| {
                HttpResponse::MethodNotAllowed()
            }))
    );
}

fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/app")
            .route(web::get().to(|| {
                HttpResponse::Ok().body("app")
            }))
            .route(web::head().to(|| {
                HttpResponse::MethodNotAllowed()
            }))
    );
}

fn init_move_cfg() {
    HttpServer::new(|| {
        App::new()
            .configure(config)
            .service(web::scope("/api").configure(scoped_config))
            .route("/", web::get().to(|| {
                HttpResponse::Ok().body("/")
            }))
    })
        .bind("127.0.0.1:8088")
        .unwrap()
        .run()
        .unwrap();
}

fn main() {

    // init_share_data();

    // init_move_cfg();

    // test::test_mod();

    httpserver::httpserver::start_http_server();

}

