use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::thread;

async fn thread_info() -> impl Responder {
    let thread_current = thread::current();
    let num_threads = thread_current.name().unwrap();
    HttpResponse::Ok().body(format!("Number of threads: {}", num_threads))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/thread_info", web::get().to(thread_info))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}