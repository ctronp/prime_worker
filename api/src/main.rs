use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use std::sync::atomic::{AtomicU16, Ordering};
use std::sync::Arc;
use prime_check::IsPrime;

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("Mundo");
    format!("Hola {}!", &name)
}

async fn prime_u64(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("value").unwrap_or("invalid");
    return match name.parse::<u64>() {
        Ok(n) => {
            format!("{{is_prime: {:?}}}", n.is_prime())
        }
        Err(_) => "{\"invalid\":true}".to_string()
    };
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting our server");
    let thread_counter = Arc::new(AtomicU16::new(1));
    HttpServer::new(move || {
        println!(
            "Starting Thread {}",
            thread_counter.fetch_add(1, Ordering::SeqCst)
        );
        let thread_index = thread_counter.load(Ordering::SeqCst);
        App::new()
            .route("/", web::get().to(greet))
            .route(
                "/health",
                web::get().to(move || {
                    HttpResponse::Ok()
                        .header("thread-id", thread_index.to_string())
                        .finish()
                }),
            )
            .route("/str", web::get().to(|| async { "Hola Rust\n" }))
            .route("/{name}", web::get().to(greet))
            .route("/u64/{value}", web::get().to(prime_u64))
    })
        .bind(("127.0.0.1", 3000))?
        .run()
        .await
}
