use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use crate::common::types::Status;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Rank {
    Customer,
    Vip,
    Administrator,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<ObjectId>,
    pub store_id: Option<ObjectId>,
    pub user_id: String,
    pub email: String,
    pub password: String,
    pub rank: Rank,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserResponse {
    pub status: Status,
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RegisterRequest {
    pub user_id: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginRequest {
    pub user_id: String,
    pub password: String,
}