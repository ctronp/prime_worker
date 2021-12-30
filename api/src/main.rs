use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use prime_check;
use prime_check::IsPrime;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("primes number")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("/url/{value}")]
async fn value64(info: web::Path<u64>) -> impl Responder {
    let info = info.into_inner();
    return if info.is_prime() {
        HttpResponse::Ok().body(
            "{id:\"1\",prime:\"true\"}"
        )
    } else {
        HttpResponse::Ok().body(
            "{id:\"1\",prime:\"false\"}"
        )
    };
}

#[post("/json")]
async fn value_json(req: ) -> impl Responder {
    let info = info.into_inner();
    return if info.is_prime() {
        HttpResponse::Ok().body(
            "{id:\"1\",prime:\"true\"}"
        )
    } else {
        HttpResponse::Ok().body(
            "{id:\"1\",prime:\"false\"}"
        )
    };
}


async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}