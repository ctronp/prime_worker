use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Input {
    pub values: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Output {
    pub values: HashMap<String, String>,
}

// impl Output {
//     #[inline]
//     fn new(cap: usize) -> Self{
//         Self {
//             values: HashMap::with_capacity(cap)
//         }
//     }
// }
