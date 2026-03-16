use crate::{errors::error::AppError, state::app_state::AppState};
use shared::dto::category::{CategoryResponseDTO, CreateCategoryDTO, UpdateCategoryDTO};

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

// !Create
pub async fn create_category(
    State(state): State<AppState>,
    Json(payload): Json<CreateCategoryDTO>,
) -> Result<impl IntoResponse, AppError> {
    let category = state.category_service.create_category(payload.name).await?;

    Ok((
        StatusCode::CREATED,
        Json(CategoryResponseDTO::from(category)),
    ))
}

// !Find All
pub async fn get_categories(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let categories = state.category_service.get_categories().await?;

    let response: Vec<CategoryResponseDTO> = categories
        .into_iter()
        .map(CategoryResponseDTO::from)
        .collect();

    Ok((StatusCode::OK, Json(response)))
}

// ! Find One

pub async fn get_category(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let category = state.category_service.get_category(id).await?;

    Ok((StatusCode::OK, Json(CategoryResponseDTO::from(category))))
}

// ! Update
pub async fn update_category(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateCategoryDTO>,
) -> Result<impl IntoResponse, AppError> {
    let category = state
        .category_service
        .update_category(id, payload.name)
        .await?;

    Ok((StatusCode::OK, Json(CategoryResponseDTO::from(category))))
}

// !Delete
pub async fn delete_category(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    state.category_service.delete_category(id).await?;

    Ok(StatusCode::NO_CONTENT)
}
