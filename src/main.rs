mod entities;
mod services;
mod database;
mod controller;
mod statics;


use std::time::Duration;
use actix_web::{web, App, HttpServer};

use tokio::join;
use tokio::signal::unix::{signal, SignalKind};
use tokio::time::sleep;

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
    // let smt = v_cpus != p_cpus;

    rayon::ThreadPoolBuilder::new()
        .num_threads(v_cpus) // Rayon Worker
        .build_global()
        .unwrap();

    actix_web::rt::System::with_tokio_rt(||
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1) // Tokio Worker
            .max_blocking_threads(2) // Tokio Blocking Thread Worker
            .enable_all()
            .build()
            .unwrap()
    )
        .block_on(async move {
            let exit_h = exit_handler();
            statics::init_static();

            println!("Initializing Server");
            let to_return = HttpServer::new(|| {
                App::new()
                    .route("/primes", web::post().to(controller::primes_handler))
            }
            )
                .bind(("0.0.0.0", statics::get_port_usize()))?
                .workers(1) // Actix Worker
                .run();

            println!("Server initialized");


            join!(to_return, exit_h).0
        })
}
