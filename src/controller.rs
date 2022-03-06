use std::ops::DerefMut;

use actix_web::{HttpRequest, HttpResponse, Responder, web};

use crate::entities;
use crate::services::process_numbers;

pub async fn primes_handler(_req: HttpRequest, mut input: web::Json<entities::Input>)
                            -> impl Responder {
    let secret = match _req.headers().get("Secret") {
        Some(secret) => {
            match secret.to_str() {
                Ok(value) => {
                    if cfg!(debug_assertion) {
                        println!("Secret: {}", value);
                    };
                    value
                }
                _ => return HttpResponse::Unauthorized().finish(),
            }
        }
        _ => {
            return HttpResponse::Unauthorized().body("Unauthorized");
        }
    };
    if !crate::statics::check_header(secret) {
        return HttpResponse::Unauthorized().body("Unauthorized");
    }


    let output = match tokio::task::spawn_blocking(move || {
        process_numbers(input.values.deref_mut())
    })
        .await
    {
        Ok(output) => {
            match output {
                Some(output) => output,
                None => {
                    return HttpResponse::BadRequest()
                        .body(r##"{"Error":"max 20 values"}"##);
                }
            }
        }
        Err(_) => {
            return HttpResponse::InternalServerError()
                .body(r##"{"Error":"Internal Server Error"}"##);
        }
    };

    match serde_json::to_string(&output) {
        Ok(body) => HttpResponse::Ok().body(body),
        Err(_) => {
            HttpResponse::InternalServerError().body(r##"{"Error":"Internal Server Error"}"##)
        }
    }
}

