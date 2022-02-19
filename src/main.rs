use rug::Integer;
use serde::{Serialize, Deserialize};


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

#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    let func = handler_fn(handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn handler(event: String, _: Context) -> Result<Output, Error> {
    if event.len() < 2001 {
        Ok(prime_b10(event))
    } else {
        Ok(Output::new('E'))
    }
}
