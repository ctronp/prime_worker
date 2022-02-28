#[cfg(test)]
#[cfg(test)]
mod integration_tests {
    use crate::entities::{Input, Output};
    use crate::statics::*;
    use actix_web::http;
    use std::collections::HashMap;
    use std::time::Duration;
    use tokio::sync::OnceCell;

    async fn initialize() {
        static INIT: OnceCell<()> = OnceCell::const_new();

        INIT.get_or_init(|| async {
            std::thread::spawn(crate::main);
            tokio::time::sleep(Duration::from_secs(10)).await;
        })
        .await;
    }

    #[actix_web::test]
    async fn test_index() {
        initialize().await;

        let resp = reqwest::get(format!("http://0.0.0.0:{}/", get_port_u16()))
            .await
            .unwrap();
        pretty_assertions::assert_eq!(http::StatusCode::OK, resp.status());
    }

    #[actix_web::test]
    async fn small_primes() {
        initialize().await;

        let res = reqwest::Client::new()
            .post(format!("http://0.0.0.0:{}/primes", get_port_u16()))
            .json(&Input {
                values: vec!["1".to_string(), "2".to_string(), "3".to_string()],
            })
            .send()
            .await
            .unwrap();

        let output = res.json::<Output>().await.unwrap();
        assert_eq!(
            output.values,
            HashMap::from([
                ("1".to_string(), "No".to_string()),
                ("2".to_string(), "Yes".to_string()),
                ("3".to_string(), "Yes".to_string()),
            ])
        )
    }
}
