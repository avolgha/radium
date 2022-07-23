use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Mirrors {
    pub mirrors: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MetaFile {
    pub packages: HashMap<String, String>,
}
