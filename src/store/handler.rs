use std::sync::Arc;
use axum::{Extension, Json, Router};
use axum::extract::Path;
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::routing::{delete, post};
use crate::common::response::ErrorResponse;
use crate::common::types::Status;
use crate::database::MongoRepository;
use crate::store::model::{StoreResponse, Store};
use crate::user::helper::validate_security_key;

pub fn store_routes() -> Router {
    Router::new()
        .route("/api/store/:store_name", post(create_store))
        .route("/api/store/:store_name", delete(delete_store))
}

pub async fn create_store(
    headers: HeaderMap,
    Path(store_name): Path<String>,
    Extension(mongo_repo): Extension<Arc<MongoRepository>>,
) -> Result<impl IntoResponse, StatusCode> {
    if let Err(err) = validate_security_key(&headers) {
        return Ok(err.into_response());
    }

    if let Some(_) = mongo_repo.find_store_by_name(&store_name).await {
        let error_response = ErrorResponse {
            status: Status::Failure,
            message: "이미 사용중인 스토어 이름입니다.".to_string(),
        };
        return Ok((StatusCode::CONFLICT, Json(error_response)).into_response());
    }

    let store = Store::new(store_name.to_string());

    let result = mongo_repo.create_store(store.clone()).await;

    match result {
        Ok(_) => {
            let response = StoreResponse {
                status: Status::Success,
                store,
            };
            Ok((StatusCode::CREATED, Json(response)).into_response())
        }
        Err(_) => {
            let error_response = ErrorResponse {
                status: Status::Error,
                message: "스토어 생성을 실패하였습니다.".to_string(),
            };
            Ok((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response())
        }
    }
}

pub async fn delete_store(
    headers: HeaderMap,
    Path(store_name): Path<String>,
    Extension(mongo_repo): Extension<Arc<MongoRepository>>,
) -> Result<impl IntoResponse, StatusCode> {
    if let Err(err) = validate_security_key(&headers) {
        return Ok(err.into_response());
    }

    let result = mongo_repo.delete_store(&store_name).await;

    match result {
        Ok(true) => {
            let response = ErrorResponse {
                status: Status::Success,
                message: format!("스토어 '{}' 삭제 성공.", store_name),
            };
            Ok((StatusCode::OK, Json(response)).into_response())
        }
        Ok(false) => {
            let error_response = ErrorResponse {
                status: Status::Failure,
                message: format!("스토어 '{}'를 찾을 수 없습니다.", store_name),
            };
            Ok((StatusCode::NOT_FOUND, Json(error_response)).into_response())
        }
        Err(_) => {
            let error_response = ErrorResponse {
                status: Status::Error,
                message: "스토어 삭제에 실패하였습니다.".to_string(),
            };
            Ok((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response())
        }
    }
}