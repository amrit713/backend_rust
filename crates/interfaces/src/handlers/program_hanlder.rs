use crate::{errors::error::AppError, state::app_state::AppState};

use shared::dto::program::{CreateProgramDTO, ProgramResponseDTO, UpdateProgramDTO};

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

// !Create
#[utoipa::path(
    post,
    path = "/api/programs",
    request_body = CreateProgramDTO,
    responses(
        (status = 201, description = "Program created successfully", body = ProgramResponseDTO)
    )
)]
pub async fn create_program(
    State(state): State<AppState>,
    Json(payload): Json<CreateProgramDTO>,
) -> Result<impl IntoResponse, AppError> {
    let program = state.program_service.create_program(payload).await?;

    Ok((StatusCode::CREATED, Json(ProgramResponseDTO::from(program))))
}

// !Get All
#[utoipa::path(
    get,
    path = "/api/programs", 
    responses(
        (status = 200, description = "List all programs", body = [ProgramResponseDTO])
    )
)]
pub async fn get_programs(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let programs = state.program_service.get_programs().await?;

    let response: Vec<ProgramResponseDTO> =
        programs.into_iter().map(ProgramResponseDTO::from).collect();

    Ok((StatusCode::OK, Json(response)))
}

// Get One
#[utoipa::path(
    get,
    path = "/api/programs/{id}",
    responses(
        (status = 200, description = "Program found", body = ProgramResponseDTO),
        (status = 404, description = "Program not found")
    ),
    params(
        ("id" = Uuid, Path, description = "Program database identifier")
    )
)]
pub async fn get_program(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let program = state.program_service.get_program(id).await?;

    Ok((StatusCode::OK, Json(ProgramResponseDTO::from(program))))
}

// !Update
#[utoipa::path(
    patch,
    path = "/api/programs/{id}",
    request_body = UpdateProgramDTO,
    responses(
        (status = 200, description = "Program updated", body = ProgramResponseDTO),
        (status = 404, description = "Program not found")
    ),
    params(
        ("id" = Uuid, Path, description = "Program database identifier")
    )
)]
pub async fn update_program(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateProgramDTO>,
) -> Result<impl IntoResponse, AppError> {
    let program = state.program_service.update_program(id, payload).await?;

    Ok((StatusCode::OK, Json(ProgramResponseDTO::from(program))))
}

// !DELETE
#[utoipa::path(
    delete,
    path = "/api/programs/{id}",
    responses(
        (status = 204, description = "Program deleted successfully"),
        (status = 404, description = "Program not found")
    ),
    params(
        ("id" = Uuid, Path, description = "Program database identifier")
    )
)]
pub async fn delete_program(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    state.program_service.delete_program(id).await?;

    Ok(StatusCode::NO_CONTENT)
}
