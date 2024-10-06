use std::env;
use dotenv::dotenv;
use once_cell::sync::Lazy;

pub struct Config {
    pub server_port: String,
    pub database_url: String,
    pub database_name: String,
    pub jwt_secret: String,
    pub resend_api_key: String,
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    dotenv().ok();

    let server_port = env::var("SERVER_PORT").expect("SERVER_PORT가 설정되지 않았습니다.");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL이 설정되지 않았습니다.");
    let database_name = env::var("DATABASE_NAME").expect("DATABASE_NAME이 설정되지 않았습니다.");
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET가 설정되지 않았습니다.");
    let resend_api_key = env::var("RESEND_API_KEY").expect("RESEND_API_KEY가 설정되지 않았습니다.");

    Config {
        server_port,
        database_url,
        database_name,
        jwt_secret,
        resend_api_key,
    }
});