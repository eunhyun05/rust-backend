use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Store {
    pub name: String,
    pub create_at: DateTime<Utc>,
    pub update_at: DateTime<Utc>,
}

impl Store {
    pub fn new(name: String) -> Self {
        let now = Utc::now();
        Store {
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
pub struct CreateStoreRequest {
    pub name: String,
}
