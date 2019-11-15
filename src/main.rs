#[macro_use]
extern crate actix_web;

use actix_web::{guard, web, middleware, dev, http, App, HttpResponse, HttpServer, Responder, Result};
use actix_web::middleware::errhandlers::{ErrorHandlerResponse, ErrorHandlers};
use listenfd::ListenFd;

mod handler;
mod model;

// global data
struct AppState {
    app_name: String,
}

fn main() {

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // auto-reloading
    // run: systemfd --no-pid -s http::8088 -- cargo watch -x run
    let mut listenfd = ListenFd::from_env();

    let mut server = HttpServer::new(|| {
        App::new()
            .data(AppState {
                app_name: "Webapp".to_string(),
            })
            // 使用默认日志格式
            .wrap(middleware::Logger::default())
            // 设置全局响应头
            .wrap(middleware::DefaultHeaders::new().header("x-version", "1.0"))
            // 指定错误处理
            .wrap(
                ErrorHandlers::new()
                    .handler(http::StatusCode::INTERNAL_SERVER_ERROR, h500)
            )
            // 业务映射
            .service(web::scope("/app")
                .service(web::scope("/user")
                    .route("/", web::get().to(handler::index))
                    .route("add", web::post().to(handler::user_add))
                )
            )

            .default_service(
                web::resource("")
                    .route(web::get().to(h404))
                    // all requests that are not `GET`
                    .route(
                        web::route()
                            .guard(guard::Not(guard::Get()))
                            .to(HttpResponse::MethodNotAllowed),
                    ),
            )
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l).unwrap()
    } else {
        server.bind("127.0.0.1:8088").unwrap()
    };

    server.run().unwrap();
}

// 404 handler
fn h404() -> impl Responder {
    model::APIResult::new(404, None::<u32>, Some("Not found".to_string()))
}

// 500 handler
fn h500<B>(mut res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    res.response_mut().headers_mut().insert(
        http::header::CONTENT_TYPE,
            http::HeaderValue::from_static("Error"),
    );
    Ok(ErrorHandlerResponse::Response(res))
}
