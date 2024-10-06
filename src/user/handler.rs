use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use bcrypt::{hash, DEFAULT_COST};
use std::sync::Arc;
use axum::extract::Extension;
use crate::common::jwt::generate_jwt;
use crate::common::types::Status;
use crate::database::MongoRepository;
use crate::common::response::ErrorResponse;
use crate::config::Config;
use crate::user::model::{User, UserResponse};

pub async fn register_user(
    Extension(mongo_repo): Extension<Arc<MongoRepository>>,
    Json(mut body): Json<User>,
) -> Result<impl IntoResponse, StatusCode> {
    if let Some(_) = mongo_repo.find_user_by_email(&body.email).await {
        let error_response = ErrorResponse {
            status: Status::Failure,
            message: String::from("이미 가입된 이메일입니다."),
        };
        return Ok((StatusCode::CONFLICT, Json(error_response)).into_response());
    }

    if let Some(_) = mongo_repo.find_user_by_username(&body.username).await {
        let error_response = ErrorResponse {
            status: Status::Failure,
            message: String::from("이미 사용중인 유저이름입니다."),
        };
        return Ok((StatusCode::CONFLICT, Json(error_response)).into_response());
    }

    let hashed_password = match hash(&body.password, DEFAULT_COST) {
        Ok(hashed) => hashed,
        Err(_) => {
            let error_response = ErrorResponse {
                status: Status::Error,
                message: "비밀번호를 해시하는 과정에 오류가 발생하였습니다.".to_string(),
            };
            return Ok((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response());
        }
    };

    body.password = hashed_password;

    let result = mongo_repo.create_user(body.clone()).await;

    match result {
        Ok(_) => {
            let token = generate_jwt(&body.username, &*Config::from_env().jwt_secret);

            let response = UserResponse {
                status: Status::Success,
                token,
            };
            Ok((StatusCode::OK, Json(response)).into_response())
        }
        Err(_) => {
            let error_response = ErrorResponse {
                status: Status::Error,
                message: "유저 생성에 실패하였습니다.".to_string(),
            };
            Ok((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response())
        }
    }
}