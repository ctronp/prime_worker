use lambda_runtime::{Context, Error, handler_fn};

use prime_check::IsPrime;
use rug::Integer;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
struct Output {
    pub prime: char,
    request_id: String,
}

impl Output {
    #[inline]
    fn new(prime: char, request_id: String) -> Self {
        Output {
            prime,
            request_id,
        }
    }
}

#[inline]
fn prime_u64(str_value: String, request_id: String) -> Output {
    match str_value.parse::<u64>() {
        Ok(n) => {
            match n.is_prime() {
                true => Output::new('T', request_id),
                false => Output::new('F', request_id)
            }
        }
        Err(_) => Output::new('E', request_id)
    }
}

#[inline]
fn prime_b10(str_value: String, request_id: String) -> Output {
    match str_value.parse::<Integer>() {
        Ok(n) => {
            match n.is_probably_prime(51) != rug::integer::IsPrime::No {
                true => Output::new('Y', request_id),
                false => Output::new('F', request_id)
            }
        }
        Err(_) => Output::new('F', request_id)
    }
}

#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    let func = handler_fn(handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn handler(event: String, context: Context) -> Result<Output, Error> {
    if event.len() < 19 {
        Ok(prime_u64(event, context.request_id))
    } else if event.len() < 2001 {
        Ok(prime_b10(event, context.request_id))
    } else {
        Ok(Output::new('E', context.request_id))
    }
}
