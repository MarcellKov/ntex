use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub name_claim: String,
    pub exp: usize,
    pub usertype_claim: String,
}
