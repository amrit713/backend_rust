use chrono::{DateTime, Utc};
use domain::category::entity::Category;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateCategoryDTO {
    pub name: String,
}

#[derive(Deserialize)]
pub struct UpdateCategoryDTO {
    pub name: String,
}

#[derive(Serialize)]
pub struct CategoryResponseDTO {
    pub id: Uuid,
    name: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<Category> for CategoryResponseDTO {
    fn from(category: Category) -> Self {
        Self {
            id: category.id,
            name: category.name,
            created_at: category.created_at,
            updated_at: category.updated_at,
        }
    }
}
