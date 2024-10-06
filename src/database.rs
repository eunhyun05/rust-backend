use mongodb::{Client, Collection};
use mongodb::error::Result;
use crate::config::CONFIG;
use crate::store::model::Store;
use crate::user::model::User;

#[derive(Clone)]
pub struct MongoRepository {
    pub user_collection: Collection<User>,
    pub store_collection: Collection<Store>,
}

impl MongoRepository {
    pub async fn init() -> Result<Self> {
        let client = Client::with_uri_str(&CONFIG.database_url).await?;
        let database = client.database(&CONFIG.database_name);
        let user_collection = database.collection::<User>("users");
        let store_collection = database.collection::<Store>("stores");
        Ok(
            MongoRepository {
                user_collection,
                store_collection
            }
        )
    }
}