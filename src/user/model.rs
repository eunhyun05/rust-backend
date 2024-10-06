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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}