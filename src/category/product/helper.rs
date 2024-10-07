use std::sync::Arc;
use axum::http::StatusCode;
use axum::{Json};
use crate::common::response::ErrorResponse;
use crate::common::types::Status;
use crate::database::MongoRepository;
use crate::category::model::Category;

pub async fn get_category_from_store(
    store_id: &Option<bson::oid::ObjectId>,
    category_name: &str,
    mongo_repo: &Arc<MongoRepository>,
) -> Result<Category, (StatusCode, Json<ErrorResponse>)> {
    match mongo_repo.find_category_by_name(store_id.unwrap(), category_name).await {
        Some(category) => Ok(category),
        None => {
            let error_response = ErrorResponse {
                status: Status::Failure,
                message: format!("카테고리 '{}'를 찾을 수 없습니다.", category_name),
            };
            Err((StatusCode::NOT_FOUND, Json(error_response)))
        }
    }
}

pub fn find_product_in_category<'a>(
    category: &'a mut Category,
    product_name: &str,
) -> Result<&'a mut crate::category::product::model::Product, (StatusCode, Json<ErrorResponse>)> {
    let product = category.products.iter_mut().find(|p| p.name == product_name);
    match product {
        Some(p) => Ok(p),
        None => {
            let error_response = ErrorResponse {
                status: Status::Failure,
                message: format!("제품 '{}'를 찾을 수 없습니다.", product_name),
            };
            Err((StatusCode::NOT_FOUND, Json(error_response)))
        }
    }
}