use axum::http::{HeaderMap, StatusCode};
use axum::{Json, Router};
use axum::response::IntoResponse;
use bcrypt::{hash, verify, DEFAULT_COST};
use std::sync::Arc;
use axum::extract::Extension;
use axum::routing::post;
use crate::common::jwt::generate_jwt;
use crate::common::types::Status;
use crate::database::MongoRepository;
use crate::common::response::ErrorResponse;
use crate::config::CONFIG;
use crate::store::helper::get_store_from_headers;
use crate::user::model::{LoginRequest, Rank, RegisterRequest, User, UserResponse};

pub fn user_routes() -> Router {
    Router::new()
        .route("/api/auth/register", post(register_user))
        .route("/api/auth/login", post(login_user))
}

pub async fn register_user(
    headers: HeaderMap,
    Extension(mongo_repo): Extension<Arc<MongoRepository>>,
    Json(body): Json<RegisterRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let store = match get_store_from_headers(&headers, &mongo_repo).await {
        Ok(store) => store,
        Err(err) => return Ok(err.into_response()),
    };

    if body.password != body.confirm_password {
        let error_response = ErrorResponse {
            status: Status::Failure,
            message: "비밀번호와 비밀번호 확인이 일치하지 않습니다.".to_string(),
        };
        return Ok((StatusCode::BAD_REQUEST, Json(error_response)).into_response());
    }

    if let Some(_) = mongo_repo.find_user_by_email(&store.object_id.unwrap(), &body.email).await {
        let error_response = ErrorResponse {
            status: Status::Failure,
            message: "이미 가입된 이메일입니다.".to_string(),
        };
        return Ok((StatusCode::CONFLICT, Json(error_response)).into_response());
    }

    if let Some(_) = mongo_repo.find_user_by_user_id(&store.object_id.unwrap(), &body.user_id).await {
        let error_response = ErrorResponse {
            status: Status::Failure,
            message: "이미 사용중인 유저 아이디입니다.".to_string(),
        };
        return Ok((StatusCode::CONFLICT, Json(error_response)).into_response());
    }

    let hashed_password = match hash(&body.password, DEFAULT_COST) {
        Ok(hashed) => hashed,
        Err(_) => {
            let error_response = ErrorResponse {
                status: Status::Error,
                message: "비밀번호 해시화에 실패했습니다.".to_string(),
            };
            return Ok((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response());
        }
    };

    let new_user = User {
        store_id: store.object_id,
        object_id: None,
        user_id: body.user_id.clone(),
        email: body.email.clone(),
        password: hashed_password,
        rank: Rank::Customer,
    };

    let result = mongo_repo.create_user(new_user.clone()).await;

    match result {
        Ok(_) => {
            let token = generate_jwt(result.unwrap().to_string().parse().unwrap(), &CONFIG.jwt_secret);

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
    headers: HeaderMap,
    Extension(mongo_repo): Extension<Arc<MongoRepository>>,
    Json(body): Json<LoginRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let store = match get_store_from_headers(&headers, &mongo_repo).await {
        Ok(store) => store,
        Err(err) => return Ok(err.into_response()),
    };

    let user = match mongo_repo.find_user_by_user_id(&store.object_id.unwrap(), &body.user_id).await {
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

    let user_object_id = match user.object_id {
        Some(id) => id,
        None => {
            let error_response = ErrorResponse {
                status: Status::Error,
                message: "유저 정보가 유효하지 않습니다.".to_string(),
            };
            return Ok((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response());
        }
    };

    let token = generate_jwt(user_object_id, &CONFIG.jwt_secret);

    let response = UserResponse {
        status: Status::Success,
        token,
    };

    Ok((StatusCode::OK, Json(response)).into_response())
}