use std::sync::Arc;
use axum::extract::Path;
use axum::http::{HeaderMap, StatusCode};
use axum::{Extension, Json, Router};
use axum::response::IntoResponse;
use axum::routing::post;
use crate::category::product::model::{CreateProductRequest, Product, ProductResponse};
use crate::common::response::ErrorResponse;
use crate::common::types::Status;
use crate::database::MongoRepository;
use crate::store::helper::get_store_from_headers;

pub fn category_product_routes() -> Router {
    Router::new()
        .route("/api/category/:category_name/product", post(create_product))
}

pub async fn create_product(
    headers: HeaderMap,
    Path(category_name): Path<String>,
    Extension(mongo_repo): Extension<Arc<MongoRepository>>,
    Json(body): Json<CreateProductRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let store = match get_store_from_headers(&headers, &mongo_repo).await {
        Ok(store) => store,
        Err(err) => return Ok(err.into_response()),
    };

    let category = match mongo_repo
        .find_category_by_name(store.object_id.unwrap(), &category_name)
        .await {
        Some(category) => category,
        None => {
            let error_response = ErrorResponse {
                status: Status::Failure,
                message: format!("카테고리 '{}'를 찾을 수 없습니다.", category_name),
            };
            return Ok((StatusCode::NOT_FOUND, Json(error_response)).into_response());
        }
    };

    if category.products.iter().any(|p| p.name == body.name) {
        let error_response = ErrorResponse {
            status: Status::Failure,
            message: format!("이미 '{}' 제품이 존재합니다.", body.name),
        };
        return Ok((StatusCode::CONFLICT, Json(error_response)).into_response());
    }

    let product = Product::new(body.name.clone(), body.description.clone(), body.price);

    match mongo_repo
        .add_product_to_category(&store.object_id.unwrap(), &category_name, product.clone())
        .await
    {
        Ok(_) => {
            let response = ProductResponse {
                status: Status::Success,
                product,
            };
            Ok((StatusCode::CREATED, Json(response)).into_response())
        }
        Err(_) => {
            let error_response = ErrorResponse {
                status: Status::Error,
                message: "제품 추가에 실패하였습니다.".to_string(),
            };
            Ok((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response())
        }
    }
}