use rayon::prelude::*;
use rug::Integer;

use crate::entities::Output;
use crate::statics::get_max_len_usize;

/// Output a String with the answer of the Primality Test
fn prime_b10(str_value: &str) -> String {
    if str_value.len() > get_max_len_usize() {
        return "value exceed max size limit".to_string();
    }
    match str_value.parse::<Integer>() {
        Ok(n) => match n.is_probably_prime(64) {
            rug::integer::IsPrime::Yes => "Yes".to_string(),
            rug::integer::IsPrime::Probably => "Probably".to_string(),
            rug::integer::IsPrime::No => "No".to_string(),
        },
        Err(_) => "Value Error".to_string(),
    }
}

/// needs to be re-written, it's not efficient and it's not using the database or async
fn process_value(value: &mut String) -> (String, String) {
    let val_ref = &value[..];
    (
        value.to_string(),
        if let Some(query) = answer_db(val_ref) {
            query
        } else {
            prime_b10(value)
        },
    )
}

pub fn process_numbers(input: &mut [String]) -> Option<Output> {
    if input.len() > 20 {
        return None;
    }
    Some(Output {
        values: input.into_par_iter().map(process_value).collect(),
    })
}
