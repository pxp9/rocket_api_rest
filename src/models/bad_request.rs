use serde::{Serialize , Deserialize};
#[derive(Serialize, Deserialize)]
pub struct BadRequest {
    pub code : u16,
    pub description : String
}