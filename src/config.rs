use std::env;
use dotenv::dotenv;

pub struct Config {
    pub server_port: String,
    pub database_url: String,
    pub database_name: String,
    pub jwt_secret: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();

        let server_port = env::var("SERVER_PORT").expect("SERVER_PORT가 설정되지 않았습니다.");
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL이 설정되지 않았습니다.");
        let database_name = env::var("DATABASE_NAME").expect("DATABASE_NAME이 설정되지 않았습니다.");
        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET가 설정되지 않았습니다.");

        Config {
            server_port,
            database_url,
            database_name,
            jwt_secret,
        }
    }
}