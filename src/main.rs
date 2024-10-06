mod database;
mod user;
mod common;
mod config;

use std::sync::Arc;
use axum::{routing::post, Extension, Router};
use crate::config::Config;
use crate::database::MongoRepository;
use crate::user::handler::register_user;

#[tokio::main]
async fn main() {
    let mongo_repo = MongoRepository::init().await.expect("MongoDB 초기화를 실패하였습니다.");

    let app = Router::new()
        .route("/api/auth/register", post(register_user))
        .layer(Extension(Arc::new(mongo_repo)));

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", &Config::from_env().server_port))
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}