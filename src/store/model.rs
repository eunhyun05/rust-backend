use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::common::types::Status;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Store {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<ObjectId>,
    pub name: String,
    pub create_at: DateTime<Utc>,
    pub update_at: DateTime<Utc>,
}

impl Store {
    pub fn new(name: String) -> Self {
        let now = Utc::now();
        Store {
            object_id: None,
            name,
            create_at: now,
            update_at: now,
        }
    }

    #[allow(dead_code)]
    pub fn update(&mut self) {
        let now = Utc::now();
        self.update_at = now;
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StoreResponse {
    pub status: Status,
    pub store: Store,
}