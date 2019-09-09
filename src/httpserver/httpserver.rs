use actix_rt;
use actix_web::{HttpServer, App, web, HttpResponse};

pub fn start_http_server() {

    let sys = actix_rt::System::new("example");
    HttpServer::new(|| {
        App::new().route("/", web::get().to(|| {
            HttpResponse::Ok().body("hello rust")
        }))
    }).bind("127.0.0.1:8088")
        .unwrap()
        .start();

    let _ = sys.run();
}




