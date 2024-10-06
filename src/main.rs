mod database;
mod user;
mod common;
mod config;

use std::sync::Arc;
use axum::{Extension, Router};
use crate::config::Config;
use crate::database::MongoRepository;

#[tokio::main]
async fn main() {
    let mongo_repo = MongoRepository::init().await.expect("MongoDB 초기화를 실패하였습니다.");

    let app = Router::new()
        .merge(user::handler::user_routes())
        .layer(Extension(Arc::new(mongo_repo)));

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", &Config::from_env().server_port))
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}