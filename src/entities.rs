use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Input {
    pub values: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Output {
    pub values: HashMap<String, String>,
}
