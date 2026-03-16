use chrono::{DateTime, Utc};

use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Category {
    pub id: Uuid,
    pub name: String,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// method
impl Category {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,

            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
