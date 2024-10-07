use bson::{doc, to_bson};
use bson::oid::ObjectId;
use mongodb::error::Result;
use crate::database::MongoRepository;
use crate::category::model::{Category, Product};

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

    pub async fn add_product_to_category(&self, category_name: &str, product: Product) -> Result<()> {
        let filter = doc! { "name": category_name };
        let product_bson = to_bson(&product)?;
        let update = doc! { "$push": { "products": product_bson } };
        self.category_collection.update_one(filter, update).await?;
        Ok(())
    }

    pub async fn remove_product_from_category(&self, category_name: &str, product_id: ObjectId) -> Result<()> {
        let filter = doc! { "name": category_name };
        let update = doc! { "$pull": { "products": { "_id": product_id } } };
        self.category_collection.update_one(filter, update).await?;
        Ok(())
    }

    pub async fn find_product_in_category(&self, category_name: &str, product_id: ObjectId) -> Option<Product> {
        let filter = doc! {
            "name": category_name,
            "products._id": product_id
        };
        let category: Option<Category> = self.category_collection.find_one(filter).await.unwrap_or(None);

        category.and_then(|c| c.products.into_iter().find(|p| p.object_id == Some(product_id)))
    }
}