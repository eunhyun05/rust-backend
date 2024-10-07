use bson::doc;
use bson::oid::ObjectId;
use mongodb::error::Result;
use crate::database::MongoRepository;
use crate::category::model::{Category};

#[allow(dead_code)]
impl MongoRepository {
    pub async fn create_category(&self, new_category: Category) -> Result<ObjectId> {
        let category = self.category_collection.insert_one(new_category).await?;
        Ok(category.inserted_id.as_object_id().unwrap())
    }

    pub async fn delete_category(&self, store_id: ObjectId, category_name: &str) -> Result<bool> {
        let filter = doc! { "store_id": store_id, "name": category_name };
        let result = self.category_collection.delete_one(filter).await?;
        Ok(result.deleted_count > 0)
    }

    pub async fn find_category_by_name(&self, store_id: ObjectId, name: &str) -> Option<Category> {
        let filter = doc! { "store_id": store_id, "name": name };
        self.category_collection.find_one(filter).await.unwrap_or(None)
    }
}