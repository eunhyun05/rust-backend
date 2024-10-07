use bson::{doc, to_bson};
use bson::oid::ObjectId;
use crate::category::model::Category;
use crate::category::product::model::Product;
use crate::database::MongoRepository;

#[allow(dead_code)]
impl MongoRepository {
    pub async fn add_product_to_category(
        &self,
        store_id: &ObjectId,
        category_name: &str,
        product: Product,
    ) -> mongodb::error::Result<()> {
        let filter = doc! { "store_id": store_id, "name": category_name };
        let product_bson = to_bson(&product)?;
        let update = doc! { "$push": { "products": product_bson } };
        self.category_collection.update_one(filter, update).await?;
        Ok(())
    }

    pub async fn remove_product_from_category(
        &self,
        store_id: &ObjectId,
        category_name: &str,
        product_id: ObjectId,
    ) -> mongodb::error::Result<()> {
        let filter = doc! { "store_id": store_id, "name": category_name };
        let update = doc! { "$pull": { "products": { "_id": product_id } } };
        self.category_collection.update_one(filter, update).await?;
        Ok(())
    }

    pub async fn find_product_in_category(
        &self,
        store_id: &ObjectId,
        category_name: &str,
        product_id: ObjectId,
    ) -> Option<Product> {
        let filter = doc! {
            "store_id": store_id,
            "name": category_name,
            "products._id": product_id
        };
        let category: Option<Category> = self.category_collection.find_one(filter).await.unwrap_or(None);

        category.and_then(|c| c.products.into_iter().find(|p| p.object_id == Some(product_id)))
    }

    pub async fn update_product_stock(
        &self,
        store_id: &ObjectId,
        category_name: &str,
        product_name: &str,
        stock: &Vec<String>,
    ) -> mongodb::error::Result<()> {
        let filter = doc! {
            "store_id": store_id,
            "name": category_name,
            "products.name": product_name,
        };
        let update = doc! { "$set": { "products.$.stock": stock } };
        self.category_collection.update_one(filter, update).await?;
        Ok(())
    }
}