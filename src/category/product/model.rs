use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use crate::common::types::Status;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Product {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<ObjectId>,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub discount_rate: Option<f64>,
    pub stock: Vec<String>,
}

#[allow(dead_code)]
impl Product {
    pub fn new(name: String, description: String, price: f64) -> Self {
        Product {
            object_id: None,
            name,
            description,
            price,
            discount_rate: None,
            stock: vec![],
        }
    }

    pub fn final_price(&self) -> f64 {
        if let Some(rate) = self.discount_rate {
            self.price * (1.0 - rate)
        } else {
            self.price
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductResponse {
    pub status: Status,
    pub product: Product,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateProductRequest {
    pub name: String,
    pub description: String,
    pub price: f64,
}