use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,

    pub created_at: DateTime<Utc>,
    pub update_at: DateTime<Utc>,
}

// method
impl User {
    pub fn new(name: String, email: String, password: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            password,
            email,
            created_at: Utc::now(),
            update_at: Utc::now(),
        }
    }
}
