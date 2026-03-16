use axum::{Router, routing::get};

use crate::handlers::program_hanlder::{
    create_program, delete_program, get_program, get_programs, update_program,
};

use crate::state::app_state::AppState;
pub fn program_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_programs).post(create_program))
        .route(
            "/{id}",
            get(get_program)
                .patch(update_program)
                .delete(delete_program),
        )
}
