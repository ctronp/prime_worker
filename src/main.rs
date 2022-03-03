use std::time::Duration;

use actix_web::{App, HttpServer, web};
use actix_web::middleware::Logger;
use tokio::join;
use tokio::signal::unix::{signal, SignalKind};
use tokio::time::sleep;

mod controller;
mod database;
mod entities;
mod services;
mod statics;
mod test;

async fn exit_handler() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = signal(SignalKind::interrupt())?;
    stream.recv().await;
    println!("Shutting Down in 20 sec");
    sleep(Duration::from_secs(20)).await;
    println!("Bye");
    Ok(())
}

fn main() -> std::io::Result<()> {
    let v_cpus = num_cpus::get();
    let p_cpus = num_cpus::get_physical();
    let smt = v_cpus != p_cpus;
    let cpus = v_cpus;

    if smt {
        println!("smt (hyper-threading) enabled")
    } else {
        println!("smt (hyper-threading) disabled")
    }

    // Log to stdout
    let mut log_builder = env_logger::Builder::from_default_env();
    log_builder.target(env_logger::Target::Stderr).init();

    rayon::ThreadPoolBuilder::new()
        .num_threads(cpus) // Rayon Worker
        .build_global()
        .unwrap();

    actix_web::rt::System::with_tokio_rt(|| {
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads((cpus as f64).cbrt() as usize + 1) // Tokio Worker
            .max_blocking_threads((cpus as f64).sqrt() as usize + 1) // Tokio Blocking Thread Worker
            .enable_all()
            .build()
            .unwrap()
    })
    .block_on(async move {
        let exit_h = exit_handler();
        statics::init_static().await;

        println!("Initializing Server");
        let to_return = HttpServer::new(|| {
            App::new()
                .wrap(Logger::default())
                .route("/", web::get().to(|| async { "/" }))
                .route("/primes", web::post().to(controller::primes_handler))
        })
        .bind(("0.0.0.0", statics::get_port_u16()))?
        .workers((cpus as f64).cbrt() as usize) // Actix Worker
        .run();

        println!("Server initialized");

        join!(to_return, exit_h).0
    })
}
