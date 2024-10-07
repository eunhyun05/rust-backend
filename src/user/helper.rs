use std::str::FromStr;
use std::sync::Arc;
use axum::http::HeaderMap;
use axum::{Json};
use axum::http::StatusCode;
use bson::oid::ObjectId;
use crate::common::jwt::validate_jwt;
use crate::common::response::ErrorResponse;
use crate::common::types::Status;
use crate::config::CONFIG;
use crate::database::MongoRepository;
use crate::store::helper::get_store_from_headers;
use crate::user::model::Rank;

pub fn validate_security_key(headers: &HeaderMap) -> Result<(), (StatusCode, Json<ErrorResponse>)> {
    let security_key = headers.get("X-Vronix-Security").and_then(|h| h.to_str().ok());
    if let Some(key) = security_key {
        if key != CONFIG.vronix_security_key {
            let error_response = ErrorResponse {
                status: Status::Failure,
                message: "유효하지 않은 보안 키입니다.".to_string(),
            };
            return Err((StatusCode::UNAUTHORIZED, Json(error_response)));
        }
    } else {
        let error_response = ErrorResponse {
            status: Status::Failure,
            message: "보안 키가 제공되지 않았습니다.".to_string(),
        };
        return Err((StatusCode::BAD_REQUEST, Json(error_response)));
    }

    Ok(())
}

pub fn validate_authorization(
    headers: &HeaderMap,
    expected_store_id: &str,
) -> Result<(String, String), (StatusCode, Json<ErrorResponse>)> {
    if let Some(auth_header) = headers.get("Authorization").and_then(|h| h.to_str().ok()) {
        if auth_header.starts_with("Bearer ") {
            let token = auth_header.trim_start_matches("Bearer ").trim();
            return match validate_jwt(token) {
                Ok(claims) => {
                    if claims.claims.store_id == expected_store_id {
                        Ok((claims.claims.store_id, claims.claims.id))
                    } else {
                        Err((
                            StatusCode::UNAUTHORIZED,
                            Json(ErrorResponse {
                                status: Status::Failure,
                                message: "잘못된 스토어 접근입니다.".to_string(),
                            }),
                        ))
                    }
                }
                Err(_) => {
                    Err((
                        StatusCode::UNAUTHORIZED,
                        Json(ErrorResponse {
                            status: Status::Failure,
                            message: "유효하지 않은 토큰입니다.".to_string(),
                        }),
                    ))
                }
            };
        }
    }

    Err((
        StatusCode::BAD_REQUEST,
        Json(ErrorResponse {
            status: Status::Failure,
            message: "Authorization 헤더가 제공되지 않았습니다.".to_string(),
        }),
    ))
}

pub async fn validate_user_rank(
    headers: &HeaderMap,
    required_rank: Rank,
    mongo_repo: &Arc<MongoRepository>,
) -> Result<(), (StatusCode, Json<ErrorResponse>)> {
    let store = match get_store_from_headers(headers, mongo_repo).await {
        Ok(store) => store,
        Err(err) => return Err(err),
    };

    let (_store_id, user_id_str) = match validate_authorization(headers, &store.object_id.unwrap().to_string()) {
        Ok((store_id, user_id)) => (store_id, user_id),
        Err(err) => return Err(err),
    };

    match ObjectId::from_str(&user_id_str) {
        Ok(user_id) => {
            println!("User ID: {}", user_id);
            match mongo_repo.find_user_by_id(&store.object_id.unwrap(), &user_id).await {
                Some(user) => {
                    if user.rank >= required_rank {
                        Ok(())
                    } else {
                        Err((
                            StatusCode::FORBIDDEN,
                            Json(ErrorResponse {
                                status: Status::Failure,
                                message: "권한이 부족합니다.".to_string(),
                            }),
                        ))
                    }
                }
                None => Err((
                    StatusCode::UNAUTHORIZED,
                    Json(ErrorResponse {
                        status: Status::Failure,
                        message: "유저를 찾을 수 없습니다.".to_string(),
                    }),
                )),
            }
        }
        Err(_) => Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                status: Status::Failure,
                message: "유효하지 않은 유저 ID 형식입니다.".to_string(),
            }))),
    }
}