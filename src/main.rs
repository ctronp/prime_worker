#![allow(unused_imports)] // Delete in the future
#![allow(dead_code)] // Delete in the future

mod entities;
mod services;
mod database;
mod controller;

use entities::{Input, Output};

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


    let mut port: u16 = 8080;
    match env::var("PORT") {
        Ok(p) => {
            match p.parse::<u16>() {
                Ok(n) => { port = n; }
                Err(_e) => {}
            };
        }
        Err(_e) => {}
    };

    let to_return = HttpServer::new(||
        App::new()
    )
        .bind(("0.0.0.0", port))?
        .workers(1)
        .run();

    join!(to_return, exit_h).0
}
