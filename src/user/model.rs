use serde::{Deserialize, Serialize};
use crate::common::types::Status;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserResponse {
    pub status: Status,
    pub token: String,
}