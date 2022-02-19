use rug::Integer;
use serde::{Serialize, Deserialize};
use hyper::{Body, Response, Server};
use hyper::service::service_fn_ok;
use hyper::rt::{self, Future};
use std::env;


#[derive(Serialize, Deserialize)]
struct Output {
    pub prime: char,
}

impl Output {
    #[inline]
    fn new(prime: char) -> Self {
        Output {
            prime,
        }
    }
}


/// Return Output with char {Y, N, P} if is prime, is not, or probably
#[inline]
fn prime_b10(str_value: String) -> Output {
    match str_value.parse::<Integer>() {
        Ok(n) => {
            match n.is_probably_prime(51) {
                rug::integer::IsPrime::Yes => Output::new('Y'),
                rug::integer::IsPrime::Probably => Output::new('P'),
                rug::integer::IsPrime::No => Output::new('N')
            }
        }
        Err(_) => Output::new('E')
    }
}

fn main() {
    pretty_env_logger::init();

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
    let addr = ([0, 0, 0, 0], port).into();

    let new_service = || {
        service_fn_ok(|_| {
            let mut hello = "Hello ".to_string();
            match env::var("TARGET") {
                Ok(target) => { hello.push_str(&target); }
                Err(_e) => { hello.push_str("World") }
            };

            Response::new(Body::from(hello))
        })
    };

    let server = Server::bind(&addr)
        .serve(new_service)
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Listening on http://{}", addr);

    rt::run(server);
}

async fn handler(event: String) -> Result<Output, &'static str> {
    if event.len() < 2001 {
        Ok(prime_b10(event))
    } else {
        Ok(Output::new('E'))
    }
}
