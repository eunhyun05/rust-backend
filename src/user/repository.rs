use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::error::Result;
use crate::database::MongoRepository;
use crate::user::model::User;

impl MongoRepository {
    pub async fn create_user(&self, new_user: User) -> Result<ObjectId> {
        let user = self.user_collection.insert_one(new_user).await?;
        Ok(user.inserted_id.as_object_id().unwrap())
    }

    #[allow(dead_code)]
    pub async fn find_user_by_id(&self, store_name: &str, id: &ObjectId) -> Option<User> {
        let filter = doc! { "_id": id, "storeName": store_name };
        self.user_collection.find_one(filter).await.unwrap_or(None)
    }

    pub async fn find_user_by_email(&self, store_id: &ObjectId, email: &str) -> Option<User> {
        let filter = doc! { "storeId": store_id, "email": email };
        self.user_collection.find_one(filter).await.unwrap_or(None)
    }

    pub async fn find_user_by_user_id(&self, store_id: &ObjectId, user_id: &str) -> Option<User> {
        let filter = doc! { "storeId": store_id, "userId": user_id };
        self.user_collection.find_one(filter).await.unwrap_or(None)
    }
}