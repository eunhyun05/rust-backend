use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use crate::category::product::model::Product;
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