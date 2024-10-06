use bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::error::Result;
use crate::database::MongoRepository;
use crate::store::model::Store;

impl MongoRepository {
    pub async fn create_store(&self, new_store: Store) -> Result<ObjectId> {
        let store = self.store_collection.insert_one(new_store).await?;
        Ok(store.inserted_id.as_object_id().unwrap())
    }

    pub async fn find_store_by_name(&self, name: &str) -> Option<Store> {
        let filter = doc! { "name": name };
        self.store_collection.find_one(filter).await.unwrap_or(None)
    }
}