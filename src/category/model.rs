use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use crate::common::types::Status;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Category {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<ObjectId>,
    pub store_id: Option<ObjectId>,
    pub name: String,
    pub description: String,
    pub products: Vec<Product>,
}

#[allow(dead_code)]
impl Category {
    pub fn new(store_id: ObjectId, name: String, description: String) -> Self {
        Category {
            object_id: None,
            store_id: Some(store_id),
            name,
            description,
            products: vec![],
        }
    }

    pub fn add_product(&mut self, product: Product) {
        self.products.push(product);
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateCategoryRequest {
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryResponse {
    pub status: Status,
    pub category: Category,
}

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