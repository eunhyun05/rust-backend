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

    pub async fn find_user_by_email(&self, email: &str) -> Option<User> {
        let filter = doc! { "email": email };
        self.user_collection.find_one(filter).await.unwrap_or(None)
    }

    pub async fn find_user_by_username(&self, username: &str) -> Option<User> {
        let filter = doc! { "username": username };
        self.user_collection.find_one(filter).await.unwrap_or(None)
    }
}