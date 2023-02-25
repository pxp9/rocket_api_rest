use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BadRequest {
    pub code: u16,
    pub description: String,
}
