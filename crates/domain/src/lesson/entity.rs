use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Lesson {
    pub id: Uuid,
    pub title: String,
    pub is_project: bool,
    pub chapter_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// method
impl Lesson {
    pub fn new(title: String, chapter_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            is_project: false,
            chapter_id,

            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn with_project(title: String, is_project: bool, chapter_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            is_project,
            chapter_id,

            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
