use std::ops::{DerefMut};
use actix_web::{HttpResponse, Responder, web};

use crate::entities;
use crate::services::process_numbers;

pub async fn primes_handler(mut input: web::Json<entities::Input>) -> impl Responder {
    let output = match tokio::task::spawn_blocking(move || {
        process_numbers(input.values.deref_mut())
    }).await {
        Ok(output) => { output }
        Err(_) => {
            return HttpResponse::InternalServerError().body(
                r##"{"Error":"Internal Server Error"}"##);
        }
    };

    match serde_json::to_string(&output) {
        Ok(body) => { HttpResponse::Ok().body(body) }
        Err(_) => {
            HttpResponse::InternalServerError().body(
                r##"{"Error":"Internal Server Error"}"##)
        }
    }
}