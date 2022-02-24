use std::ops::{Deref, DerefMut};
use actix_web::{Error, FromRequest, HttpMessage, HttpRequest, HttpResponse, Responder, web};
use actix_web::web::Payload;
use crate::entities;
use crate::services::process_numbers;

pub async fn primes_handler(mut input: web::Json<entities::Input>) -> impl Responder {
    let output = process_numbers(input.values.deref_mut());

    match serde_json::to_string(&output) {
        Ok(body) => { HttpResponse::Ok().body(body) }
        Err(_) => {
            HttpResponse::InternalServerError().body(
                r##"{"Error":"Internal Server Error"}"##)
        }
    }
}