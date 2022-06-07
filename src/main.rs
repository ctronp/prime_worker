use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use env_logger::Env;
use tokio::join;

mod controller;
mod entities;
mod services;
mod statics;
mod test;

// async fn exit_handler() -> Result<(), Box<dyn std::error::Error>> {
//     let mut stream = signal(SignalKind::interrupt())?;
//     stream.recv().await;
//     println!("Shutting Down in 20 sec");
//     sleep(Duration::from_secs(20)).await;
//     println!("Bye");
//     Ok(())
// }

fn main() -> std::io::Result<()> {
    let v_cpus = num_cpus::get();
    let p_cpus = num_cpus::get_physical();
    let smt = v_cpus != p_cpus;
    let cpus = v_cpus;

    // Log to stdout
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    if smt {
        log::info!("smt (hyper-threading) enabled")
    } else {
        log::info!("smt (hyper-threading) disabled")
    }

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
        statics::init_static().await;

        log::info!("Initializing Server");
        let to_return = HttpServer::new(|| {
            App::new()
                .wrap(Logger::default())
                .route("/", web::get().to(|| async { "/" }))
                .route("/primes", web::post().to(controller::primes_handler))
        })
        .bind(("0.0.0.0", statics::get_port_u16()))?
        .workers((cpus as f64).cbrt() as usize) // Actix Worker
        .run();

        log::info!("Server initialized");

        join!(to_return).0
    })
}
