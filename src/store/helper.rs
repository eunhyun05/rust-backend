use std::sync::Arc;
use axum::http::StatusCode;
use axum::Json;
use axum_extra::headers::HeaderMap;
use crate::common::response::ErrorResponse;
use crate::common::types::Status;
use crate::database::MongoRepository;
use crate::store::model::Store;

pub async fn get_store_from_headers(
    headers: &HeaderMap,
    mongo_repo: &Arc<MongoRepository>,
) -> Result<Store, (StatusCode, Json<ErrorResponse>)> {
    let store_name = headers
        .get("X-Store-Name")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("");

    if store_name.is_empty() {
        let error_response = ErrorResponse {
            status: Status::Failure,
            message: "스토어 이름을 제공해주세요.".to_string(),
        };
        return Err((StatusCode::BAD_REQUEST, Json(error_response)));
    }

    match mongo_repo.find_store_by_name(store_name).await {
        Some(store) => Ok(store),
        None => {
            let error_response = ErrorResponse {
                status: Status::Failure,
                message: "존재하지 않는 스토어입니다.".to_string(),
            };
            Err((StatusCode::BAD_REQUEST, Json(error_response)))
        }
    }
}