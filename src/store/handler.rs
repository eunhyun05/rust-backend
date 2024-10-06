use std::sync::Arc;
use axum::{Extension, Json, Router};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum_extra::headers::Authorization;
use axum_extra::headers::authorization::Bearer;
use axum_extra::TypedHeader;
use bson::oid::ObjectId;
use crate::common::jwt::validate_jwt;
use crate::common::response::ErrorResponse;
use crate::common::types::Status;
use crate::database::MongoRepository;
use crate::store::model::{CreateStoreRequest, CreateStoreResponse, Store};
use crate::user::model::Rank;

pub fn store_routes() -> Router {
    Router::new()
        .route("/api/store", post(create_store))
}

pub async fn create_store(
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    Extension(mongo_repo): Extension<Arc<MongoRepository>>,
    Json(body): Json<CreateStoreRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let token = bearer.token();
    let claims = match validate_jwt(token) {
        Ok(claims) => claims.claims,
        Err(_) => {
            let error_response = ErrorResponse {
                status: Status::Error,
                message: "유효하지 않은 토큰입니다.".to_string(),
            };
            return Ok((StatusCode::UNAUTHORIZED, Json(error_response)).into_response());
        }
    };

    let user_id = match claims.id.parse::<ObjectId>() {
        Ok(id) => id,
        Err(_) => {
            let error_response = ErrorResponse {
                status: Status::Error,
                message: "유효하지 않은 사용자 ID입니다.".to_string(),
            };
            return Ok((StatusCode::UNAUTHORIZED, Json(error_response)).into_response());
        }
    };

    let user = match mongo_repo.find_user_by_id(&user_id).await {
        Some(user) => user,
        None => {
            let error_response = ErrorResponse {
                status: Status::Error,
                message: "유저를 찾을 수 없습니다.".to_string(),
            };
            return Ok((StatusCode::UNAUTHORIZED, Json(error_response)).into_response());
        }
    };

    if user.rank != Rank::Administrator {
        let error_response = ErrorResponse {
            status: Status::Failure,
            message: "관리자 권한이 없습니다.".to_string(),
        };
        return Ok((StatusCode::FORBIDDEN, Json(error_response)).into_response());
    }

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
            let response = CreateStoreResponse {
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