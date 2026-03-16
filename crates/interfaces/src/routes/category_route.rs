use axum::{Router, routing::get};

use crate::handlers::category_handler::{
    create_category, delete_category, get_categories, get_category, update_category,
};
use crate::state::app_state::AppState;

pub fn category_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_categories).post(create_category))
        .route(
            "/{id}",
            get(get_category)
                .patch(update_category)
                .delete(delete_category),
        )
}
