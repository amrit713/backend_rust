use chrono::{DateTime, Utc};
use domain::program::entity::{Difficulty, Program};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, serde::Deserialize, ToSchema)]
pub struct UpdateProgramDTO {
    pub title: Option<String>,
    pub description: Option<String>,
    pub duration: Option<i16>,
    pub difficulty: Option<Difficulty>,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateProgramDTO {
    pub title: String,
    pub description: Option<String>,
    pub duration: i16,
    pub difficulty: Difficulty,
    pub category_id: Uuid,
}

#[derive(Serialize, ToSchema)]
pub struct ProgramResponseDTO {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub duration: i16,
    pub difficulty: Difficulty,
    pub category_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Program> for ProgramResponseDTO {
    fn from(program: Program) -> Self {
        Self {
            id: program.id,
            title: program.title,
            description: program.description,
            duration: program.duration,
            category_id: program.category_id,
            difficulty: program.difficulty,
            created_at: program.created_at,
            updated_at: program.updated_at,
        }
    }
}
