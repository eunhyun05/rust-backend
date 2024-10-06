use serde::{Serialize, Deserialize};
use crate::common::types::Status;

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub status: Status,
    pub message: String,
}