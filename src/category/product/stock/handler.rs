use std::sync::Arc;
use axum::extract::Path;
use axum::http::{HeaderMap, StatusCode};
use axum::{Extension, Json, Router};
use axum::response::IntoResponse;
use axum::routing::{patch};
use crate::category::product::helper::{find_product_in_category, get_category_from_store};
use crate::common::response::ErrorResponse;
use crate::common::types::Status;
use crate::database::MongoRepository;
use crate::store::helper::get_store_from_headers;

pub fn stock_routes() -> Router {
    Router::new()
        .route("/api/category/:category_name/product/:product_name/stock", patch(update_stock))
}

pub async fn update_stock(
    headers: HeaderMap,
    Path((category_name, product_name)): Path<(String, String)>,
    Extension(mongo_repo): Extension<Arc<MongoRepository>>,
    Json(body): Json<Vec<String>>,
) -> Result<impl IntoResponse, StatusCode> {
    let store = match get_store_from_headers(&headers, &mongo_repo).await {
        Ok(store) => store,
        Err(err) => return Ok(err.into_response()),
    };

    let mut category = match get_category_from_store(&store.object_id, &category_name, &mongo_repo).await {
        Ok(category) => category,
        Err(err) => return Ok(err.into_response()),
    };

    let product = match find_product_in_category(&mut category, &product_name) {
        Ok(p) => p,
        Err(err) => return Ok(err.into_response()),
    };

    product.stock = body.clone();

    match mongo_repo
        .update_product_stock(&store.object_id.unwrap(), &category_name, &product_name, &product.stock)
        .await
    {
        Ok(_) => {
            let response = ErrorResponse {
                status: Status::Success,
                message: format!("제품 '{}'의 재고가 성공적으로 업데이트되었습니다.", product_name),
            };
            Ok((StatusCode::OK, Json(response)).into_response())
        }
        Err(_) => {
            let error_response = ErrorResponse {
                status: Status::Error,
                message: "재고 업데이트에 실패하였습니다.".to_string(),
            };
            Ok((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response())
        }
    }
}