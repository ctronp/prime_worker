#![allow(unused_imports)] // Delete in the future
#![allow(dead_code)] // Delete in the future

mod entities;
mod services;
mod database;
mod controller;
mod statics;

use rug::Integer;
use serde::{Serialize, Deserialize};
use std::env;
use std::process::exit;
use std::time::Duration;
use tokio::sync::mpsc::channel;

use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};
use actix_web::cookie::time::Time;
use tokio::join;
use tokio::signal::unix::{signal, SignalKind};
use tokio::time::sleep;

async fn exit_handler() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = signal(SignalKind::interrupt())?;
    stream.recv().await;
    println!("Shutting Down in 20 sec");
    sleep(Duration::from_secs(20)).await;
    exit(1)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let exit_h = exit_handler();
    statics::init_static();

    println!("Initializing Server");
    let to_return = HttpServer::new(|| {
        App::new()
            .route("/primes", web::get().to(controller::primes_handler))
    }
    )
        .bind(("0.0.0.0", statics::get_port_usize()))?
        .workers(1)
        .run();

    println!("Server initialized");

    join!(to_return, exit_h).0
}
