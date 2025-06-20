use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct KDFParams {
    pub name: String,
    pub hash: String,
    pub iterations: u32,
    pub salt: String,
}
