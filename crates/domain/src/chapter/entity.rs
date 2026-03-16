use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Chapter {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub program_id: Uuid,

    pub created_at: DateTime<Utc>,
    pub update_at: DateTime<Utc>,
}

// method
impl Chapter {
    pub fn new(title: String, description: Option<String>, program_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            description,
            program_id,

            created_at: Utc::now(),
            update_at: Utc::now(),
        }
    }
}
