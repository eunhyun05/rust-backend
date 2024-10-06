use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use bcrypt::{hash, verify, DEFAULT_COST};
use std::sync::Arc;
use axum::extract::Extension;
use crate::common::jwt::generate_jwt;
use crate::common::types::Status;
use crate::database::MongoRepository;
use crate::common::response::ErrorResponse;
use crate::config::Config;
use crate::user::model::{LoginRequest, RegisterRequest, User, UserResponse};

pub async fn register_user(
    Extension(mongo_repo): Extension<Arc<MongoRepository>>,
    Json(body): Json<RegisterRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    if body.password != body.confirm_password {
        let error_response = ErrorResponse {
            status: Status::Failure,
            message: "비밀번호와 비밀번호 확인이 일치하지 않습니다.".to_string(),
        };
        return Ok((StatusCode::BAD_REQUEST, Json(error_response)).into_response());
    }

    if let Some(_) = mongo_repo.find_user_by_email(&body.email).await {
        let error_response = ErrorResponse {
            status: Status::Failure,
            message: "이미 가입된 이메일입니다.".to_string(),
        };
        return Ok((StatusCode::CONFLICT, Json(error_response)).into_response());
    }

    if let Some(_) = mongo_repo.find_user_by_username(&body.username).await {
        let error_response = ErrorResponse {
            status: Status::Failure,
            message: "이미 사용중인 유저이름입니다.".to_string(),
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

    let new_user = User {
        username: body.username.clone(),
        email: body.email.clone(),
        password: hashed_password,
    };

    let result = mongo_repo.create_user(new_user.clone()).await;

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

pub async fn login_user(
    Extension(mongo_repo): Extension<Arc<MongoRepository>>,
    Json(body): Json<LoginRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let user = match mongo_repo.find_user_by_username(&body.username).await {
        Some(user) => user,
        None => {
            let error_response = ErrorResponse {
                status: Status::Failure,
                message: "존재하지 않는 사용자입니다.".to_string(),
            };
            return Ok((StatusCode::UNAUTHORIZED, Json(error_response)).into_response());
        }
    };

    let is_password_valid = match verify(&body.password, &user.password) {
        Ok(valid) => valid,
        Err(_) => {
            let error_response = ErrorResponse {
                status: Status::Error,
                message: "비밀번호 검증에 실패하였습니다.".to_string(),
            };
            return Ok((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response());
        }
    };

    if !is_password_valid {
        let error_response = ErrorResponse {
            status: Status::Failure,
            message: "잘못된 비밀번호입니다.".to_string(),
        };
        return Ok((StatusCode::UNAUTHORIZED, Json(error_response)).into_response());
    }

    let token = generate_jwt(&user.username, &*Config::from_env().jwt_secret);

    let response = UserResponse {
        status: Status::Success,
        token,
    };

    Ok((StatusCode::OK, Json(response)).into_response())
}