use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use prime_check::IsPrime;
use rug::Integer;

const PRIME_OUTPUT: (&str, &str, &str) = (
    "{is_prime: true}",
    "{is_prime: false}",
    "{is_prime: invalid}"
);

async fn prime_u64(req: HttpRequest) -> impl Responder {
    let str_value = req.match_info().get("value").unwrap_or("invalid");
    return match str_value.parse::<u64>() {
        Ok(n) => {
            if n.is_prime() { PRIME_OUTPUT.0 } else { PRIME_OUTPUT.1 }
        }
        Err(_) => PRIME_OUTPUT.2
    };
}

async fn prime_b10(req: HttpRequest) -> impl Responder {
    let str_value = req.match_info().get("value").unwrap_or("invalid");
    return match str_value.parse::<Integer>() {
        Ok(n) => {
            if n.is_probably_prime(50) != rug::integer::IsPrime::No {
                PRIME_OUTPUT.0
            } else { PRIME_OUTPUT.1 }
        }
        Err(_) => PRIME_OUTPUT.2
    };
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting our server");
    HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(|| async { "Hello World\n" }))
            .route("/u64/{value}", web::get().to(prime_u64))
            .route("/b10/{value}", web::get().to(prime_b10))
    })
        .bind(("127.0.0.1", 3000))?
        .run()
        .await
}
