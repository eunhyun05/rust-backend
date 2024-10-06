use mongodb::{Client, Collection};
use mongodb::error::Result;
use crate::config::Config;
use crate::user::model::User;

#[derive(Clone)]
pub struct MongoRepository {
    pub user_collection: Collection<User>,
}

impl MongoRepository {
    pub async fn init() -> Result<Self> {
        let client = Client::with_uri_str(&Config::from_env().database_url).await?;
        let database = client.database(&Config::from_env().database_name);
        let user_collection = database.collection::<User>("users");
        Ok(MongoRepository { user_collection })
    }
}
