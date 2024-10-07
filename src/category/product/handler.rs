use std::sync::Arc;
use axum::extract::Path;
use axum::http::{HeaderMap, StatusCode};
use axum::{Extension, Json, Router};
use axum::response::IntoResponse;
use axum::routing::{delete, patch, post};
use bson::oid::ObjectId;
use crate::category::product::helper::{find_product_in_category, get_category_from_store};
use crate::category::product::model::{CreateProductRequest, Product, ProductResponse};
use crate::common::response::ErrorResponse;
use crate::common::types::Status;
use crate::database::MongoRepository;
use crate::store::helper::get_store_from_headers;
use crate::user::helper::{validate_user_rank};
use crate::user::model::Rank;

pub fn product_routes() -> Router {
    Router::new()
        .route("/api/category/:category_name/product", post(create_product))
        .route("/api/category/:category_name/product/:product_name", delete(delete_product))
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

    if let Err(err) = validate_user_rank(&headers, Rank::Administrator, &mongo_repo).await {
        return Ok(err.into_response());
    }

    let category = match get_category_from_store(&store.object_id, &category_name, &mongo_repo).await {
        Ok(category) => category,
        Err(err) => return Ok(err.into_response()),
    };

    if category.products.iter().any(|p| p.name == body.name) {
        let error_response = ErrorResponse {
            status: Status::Failure,
            message: format!("이미 '{}' 제품이 존재합니다.", body.name),
        };
        return Ok((StatusCode::CONFLICT, Json(error_response)).into_response());
    }

    let mut product = Product::new(body.name.clone(), body.description.clone(), body.price);
    product.object_id = Some(ObjectId::new());

    match mongo_repo
        .add_product_to_category(&store.object_id.clone().unwrap(), &category_name, product.clone())
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

pub async fn delete_product(
    headers: HeaderMap,
    Path((category_name, product_name)): Path<(String, String)>,
    Extension(mongo_repo): Extension<Arc<MongoRepository>>,
) -> Result<impl IntoResponse, StatusCode> {
    let store = match get_store_from_headers(&headers, &mongo_repo).await {
        Ok(store) => store,
        Err(err) => return Ok(err.into_response()),
    };

    if let Err(err) = validate_user_rank(&headers, Rank::Administrator, &mongo_repo).await {
        return Ok(err.into_response());
    }

    let mut category = match get_category_from_store(&store.object_id, &category_name, &mongo_repo).await {
        Ok(category) => category,
        Err(err) => return Ok(err.into_response()),
    };

    let product = match find_product_in_category(&mut category, &product_name) {
        Ok(p) => p,
        Err(err) => return Ok(err.into_response()),
    };

    let product_id = match product.object_id {
        Some(id) => id,
        None => {
            let error_response = ErrorResponse {
                status: Status::Error,
                message: "유효하지 않은 제품 ID입니다.".to_string(),
            };
            return Ok((StatusCode::BAD_REQUEST, Json(error_response)).into_response());
        }
    };

    match mongo_repo
        .remove_product_from_category(&store.object_id.clone().unwrap(), &category_name, product_id)
        .await
    {
        Ok(_) => {
            let response = ErrorResponse {
                status: Status::Success,
                message: format!("제품 '{}' 삭제 성공.", product_name),
            };
            Ok((StatusCode::OK, Json(response)).into_response())
        }
        Err(_) => {
            let error_response = ErrorResponse {
                status: Status::Error,
                message: "제품 삭제에 실패하였습니다.".to_string(),
            };
            Ok((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response())
        }
    }
}