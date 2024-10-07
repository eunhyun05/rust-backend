use std::sync::Arc;
use axum::{Extension, Json, Router};
use axum::extract::Path;
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::routing::{delete, post};
use crate::category::model::{Category, CategoryResponse, CreateCategoryRequest};
use crate::common::response::ErrorResponse;
use crate::common::types::Status;
use crate::database::MongoRepository;
use crate::store::helper::get_store_from_headers;

pub fn category_routes() -> Router {
    Router::new()
        .route("/api/category", post(create_category))
        .route("/api/category/:category_name", delete(delete_category))
}

pub async fn create_category(
    headers: HeaderMap,
    Extension(mongo_repo): Extension<Arc<MongoRepository>>,
    Json(body): Json<CreateCategoryRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let store = match get_store_from_headers(&headers, &mongo_repo).await {
        Ok(store) => store,
        Err(err) => return Ok(err.into_response()),
    };

    if let Some(_) = mongo_repo.find_category_by_name(store.object_id.unwrap(), &body.name).await {
        let error_response = ErrorResponse {
            status: Status::Failure,
            message: "이미 존재하는 카테고리 이름입니다.".to_string(),
        };
        return Ok((StatusCode::CONFLICT, Json(error_response)).into_response());
    }

    let category = Category::new(store.object_id.unwrap(), body.name.clone(), body.description.clone());

    match mongo_repo.create_category(category.clone()).await {
        Ok(_) => {
            let response = CategoryResponse {
                status: Status::Success,
                category,
            };
            Ok((StatusCode::CREATED, Json(response)).into_response())
        }
        Err(_) => {
            let error_response = ErrorResponse {
                status: Status::Error,
                message: "카테고리 생성에 실패하였습니다.".to_string(),
            };
            Ok((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response())
        }
    }
}

pub async fn delete_category(
    headers: HeaderMap,
    Path(category_name): Path<String>,
    Extension(mongo_repo): Extension<Arc<MongoRepository>>,
) -> Result<impl IntoResponse, StatusCode> {
    let store = match get_store_from_headers(&headers, &mongo_repo).await {
        Ok(store) => store,
        Err(err) => return Ok(err.into_response()),
    };

    match mongo_repo.delete_category(store.object_id.unwrap(), &category_name).await {
        Ok(true) => {
            let response = ErrorResponse {
                status: Status::Success,
                message: format!("카테고리 '{}' 삭제 성공.", category_name),
            };
            Ok((StatusCode::OK, Json(response)).into_response())
        }
        Ok(false) => {
            let error_response = ErrorResponse {
                status: Status::Failure,
                message: format!("카테고리 '{}'를 찾을 수 없습니다.", category_name),
            };
            Ok((StatusCode::NOT_FOUND, Json(error_response)).into_response())
        }
        Err(_) => {
            let error_response = ErrorResponse {
                status: Status::Error,
                message: "카테고리 삭제에 실패하였습니다.".to_string(),
            };
            Ok((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response())
        }
    }
}