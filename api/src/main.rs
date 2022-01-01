use std::sync::Arc;
use std::sync::atomic::{AtomicU16, Ordering};
use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("Mundo");
    format!("Hola {}!", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting our server");
    let thread_counter = Arc::new(AtomicU16::new(1));
    HttpServer::new(move || {
        println!("Starting Thread {}",
                 thread_counter.fetch_add(1, Ordering::SeqCst));
        let thread_index = thread_counter.load(Ordering::SeqCst);
        App::new()
            .route("/", web::get().to(greet))
            .route("/health",
                   web::get().to(move ||
                       HttpResponse::Ok()
                           .header("thread-id", thread_index.to_string())
                           .finish()
                   ))
            .route("/str", web::get().to(|| async { "Hola Rust\n" }))
            .route("/{name}", web::get().to(greet))
    })
        .bind(("127.0.0.1", 3000))?
        .run()
        .await
}
