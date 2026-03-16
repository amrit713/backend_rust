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
pub async fn create_program(
    State(state): State<AppState>,
    Json(payload): Json<CreateProgramDTO>,
) -> Result<impl IntoResponse, AppError> {
    let program = state.program_service.create_program(payload).await?;

    Ok((StatusCode::CREATED, Json(ProgramResponseDTO::from(program))))
}

// !Get All
pub async fn get_programs(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let programs = state.program_service.get_programs().await?;

    let response: Vec<ProgramResponseDTO> =
        programs.into_iter().map(ProgramResponseDTO::from).collect();

    Ok((StatusCode::OK, Json(response)))
}

// Get One
pub async fn get_program(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let program = state.program_service.get_program(id).await?;

    Ok((StatusCode::OK, Json(ProgramResponseDTO::from(program))))
}

// !Update
pub async fn update_program(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateProgramDTO>,
) -> Result<impl IntoResponse, AppError> {
    let program = state.program_service.update_program(id, payload).await?;

    Ok((StatusCode::OK, Json(ProgramResponseDTO::from(program))))
}

pub async fn delete_program(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    state.program_service.delete_program(id).await?;

    Ok(StatusCode::NO_CONTENT)
}
