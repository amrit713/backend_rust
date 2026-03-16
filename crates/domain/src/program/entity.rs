use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, sqlx::Type)]
#[sqlx(type_name = "VARCHAR", rename_all = "PascalCase")]
pub enum Difficulty {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

// Convert Enum to String for SQL
impl AsRef<str> for Difficulty {
    fn as_ref(&self) -> &str {
        match self {
            Difficulty::Beginner => "Beginner",
            Difficulty::Intermediate => "Intermediate",
            Difficulty::Advanced => "Advanced",
            Difficulty::Expert => "Expert",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Program {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub duration: i16,
    pub difficulty: Difficulty,
    pub category_id: Uuid,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// method
impl Program {
    pub fn new(
        title: String,
        description: Option<String>,
        duration: i16,
        category_id: Uuid,
        difficulty: Difficulty,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            description,
            duration,
            difficulty,
            category_id,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
