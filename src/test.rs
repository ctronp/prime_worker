#[cfg(test)]
#[cfg(test)]
mod integration_tests {
    use std::time::Duration;
    use crate::controller::*;
    use crate::statics::*;
    use actix_web::{
        test,
        web,
        App,
        http,
    };

    async fn initialize() {
        static mut INIT: bool = false;
        unsafe {
            if !INIT {
                std::thread::spawn(crate::main);
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        }
    }

    #[actix_web::test]
    async fn test_index() {
        initialize().await;

        let resp = reqwest::get(
            format!("0.0.0.0:{}/", get_port_u16())).await.unwrap();
        pretty_assertions::assert_eq!(http::StatusCode::OK, resp.status());
    }

    // #[actix_web::test]
    // async fn test_primes_handler() {
    //     initialize().await;
    //
    //     let req = test::TestRequest::get().uri("/").to_request();
    //     let resp = test::call_and_read_body()
    // }
}

