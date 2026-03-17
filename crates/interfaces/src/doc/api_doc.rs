use crate::handlers::{category_handler, program_hanlder};
use domain::program::entity::Difficulty;
use serde::Serialize;
use shared::dto::category::{CategoryResponseDTO, CreateCategoryDTO, UpdateCategoryDTO};
use shared::dto::program::{CreateProgramDTO,ProgramResponseDTO, UpdateProgramDTO};
use utoipa::OpenApi;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct ErrorResponse {
    /// High-level error description
    pub error: String,
    /// Detailed message for debugging
    pub message: String,
    /// Technical error code (e.g., "NOT_FOUND")
    pub code: u16,
}

#[derive(OpenApi)]
#[openapi(
    paths(
        // Program Endpoints
        program_hanlder::get_programs,
        program_hanlder::get_program,
        program_hanlder::create_program,
        program_hanlder::update_program,
        program_hanlder::delete_program,
        
        // Category Endpoints
        category_handler::get_categories,
        category_handler::get_category,
        category_handler::create_category,
        category_handler::update_category,
        category_handler::delete_category,
    ),
    components(
        schemas(
            // Program Schemas
            ProgramResponseDTO, 
            CreateProgramDTO, 
            UpdateProgramDTO, 
            Difficulty
            ,
            
            // Category Schemas
            CategoryResponseDTO, 
            CreateCategoryDTO, 
            UpdateCategoryDTO,
            
            // Shared Schemas
            ErrorResponse
        )
    ),
    tags(
        (name = "Programs", description = "Operations related to educational programs"),
        (name = "Categories", description = "Operations related to program categorization")
    ),
    info(
        title = "The CNC Studio API",
        version = "1.0.0",
        description = "Backend API for managing CNC programs and categories using DDD principles."
    )
)]
pub struct ApiDoc;