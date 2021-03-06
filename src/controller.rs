use std::ops::DerefMut;

use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::entities;
use crate::services::process_numbers;

pub async fn primes_handler(
    _req: HttpRequest,
    mut input: web::Json<entities::Input>,
) -> impl Responder {
    let secret = match _req.headers().get(crate::statics::get_secret_header()) {
        Some(secret) => match secret.to_str() {
            Ok(value) => value,
            _ => return HttpResponse::Unauthorized().finish(),
        },
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
        Ok(output) => match output {
            Some(output) => output,
            None => {
                return HttpResponse::BadRequest().body(r##"{"Error":"max 20 values"}"##);
            }
        },
        Err(_) => {
            return HttpResponse::InternalServerError()
                .body(r##"{"Error":"Internal Server Error"}"##);
        }
    };

    match serde_json::to_string(&output) {
        Ok(body) => HttpResponse::Ok()
            .append_header(("Content-Type", "application/json"))
            .body(body),
        Err(_) => HttpResponse::InternalServerError()
            .append_header(("Content-Type", "application/json"))
            .body(r##"{"Error":"Internal Server Error"}"##),
    }
}
