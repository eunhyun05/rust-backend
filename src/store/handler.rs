use std::sync::Arc;
use axum::{Extension, Json, Router};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use crate::common::response::ErrorResponse;
use crate::common::types::Status;
use crate::database::MongoRepository;
use crate::store::model::{CreateStoreRequest, Store};

pub fn store_routes() -> Router {
    Router::new()
        .route("/api/store", post(create_store))
}

pub async fn create_store(
    Extension(mongo_repo): Extension<Arc<MongoRepository>>,
    Json(body): Json<CreateStoreRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    if let Some(_) = mongo_repo.find_store_by_name(&body.name).await {
        let error_response = ErrorResponse {
            status: Status::Failure,
            message: "이미 사용중인 스토어 이름입니다.".to_string(),
        };
        return Ok((StatusCode::CONFLICT, Json(error_response)).into_response());
    }

    let store = Store::new(body.name.to_string());

    let result = mongo_repo.create_store(store.clone()).await;

    match result {
        Ok(_) => {
            Ok((StatusCode::CREATED, Json(store.clone())).into_response())
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