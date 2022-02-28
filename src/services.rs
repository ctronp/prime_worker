use rug::Integer;
use rayon::prelude::*;
use crate::database::answer_db;
use crate::entities::{Output};

#[inline]
fn max_value_len() -> usize {
    unsafe {
        static mut MAX_VALUE_LEN: usize = 0;
        if MAX_VALUE_LEN == 0 {
            MAX_VALUE_LEN = crate::statics::get_max_value_usize();
        }
        MAX_VALUE_LEN
    }
}

// Return Output with char {Y, N, P} if is prime, is not, or probably
fn prime_b10(str_value: &str) -> String {
    if str_value.len() > max_value_len() {
        return "value exceed max size limit".to_string();
    }
    match str_value.parse::<Integer>() {
        Ok(n) => {
            match n.is_probably_prime(51) {
                rug::integer::IsPrime::Yes => "Yes".to_string(),
                rug::integer::IsPrime::Probably => "Probably".to_string(),
                rug::integer::IsPrime::No => "No".to_string()
            }
        }
        Err(_) => "Value Error".to_string()
    }
}

// Processed by rayon
fn process_value(value: &mut String) -> (String, String) {
    let val_ref = &value[..];
    (value.to_string(),
     if let Some(query) = answer_db(val_ref) {
         query
     } else {
         prime_b10(value)
     }
    )
}

pub fn process_numbers(input: &mut [String]) -> Output {
    Output {
        values: input.into_par_iter()
            .map(process_value)
            .collect(),

    }
}
