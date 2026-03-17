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
#[utoipa::path(
    post,
    path = "/api/categories",
    request_body = CreateCategoryDTO,
    responses((status = 201, body = CategoryResponseDTO))
)]
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

// GET ALL
#[utoipa::path(
    get,
    path = "/api/categories",
    responses((status = 200, body = [CategoryResponseDTO]))
)]
pub async fn get_categories(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let categories = state.category_service.get_categories().await?;

    let response: Vec<CategoryResponseDTO> = categories
        .into_iter()
        .map(CategoryResponseDTO::from)
        .collect();

    Ok((StatusCode::OK, Json(response)))
}

// GET ONE
#[utoipa::path(
    get,
    path = "/api/categories/{id}",
    responses((status = 200, body = CategoryResponseDTO)),
    params(("id" = Uuid, Path))
)]
pub async fn get_category(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let category = state.category_service.get_category(id).await?;

    Ok((StatusCode::OK, Json(CategoryResponseDTO::from(category))))
}

// UPDATE (PATCH)
#[utoipa::path(
    patch,
    path = "/api/categories/{id}",
    request_body = UpdateCategoryDTO,
    responses((status = 200, body = CategoryResponseDTO)),
    params(("id" = Uuid, Path))
)]
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

// DELETE
#[utoipa::path(
    delete,
    path = "/api/categories/{id}",
    responses((status = 204)),
    params(("id" = Uuid, Path))
)]
pub async fn delete_category(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    state.category_service.delete_category(id).await?;

    Ok(StatusCode::NO_CONTENT)
}
