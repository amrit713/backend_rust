use application::services::{category_service::CategoryService, program_service::ProgramService};
use infra::db::connection::create_pool;
use infra::repo::{category_repo::SqlxCategoryRepo, program_repo::SqlxProgramRepo};
use interfaces::routes::{category_route::category_routes, program_route::program_routes};
use interfaces::state::app_state::AppState;

use std::sync::Arc;

use axum::Router;
use dotenvy::dotenv;
use std::env;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URl not set");
    let port = env::var("PORT").expect("PORT not set");

    let pool = create_pool(&db_url)
        .await
        .expect("Failed to create database pool");

    let category_repo = Arc::new(SqlxCategoryRepo::new(pool.clone()));
    let program_repo = Arc::new(SqlxProgramRepo::new(pool.clone()));

    let category_service = Arc::new(CategoryService::new(category_repo.clone()));
    let program_service = Arc::new(ProgramService::new(program_repo, category_repo));

    let state = AppState {
        category_service,
        program_service,
    };

    let app: Router = Router::new()
        .nest("/api/categories", category_routes())
        .nest("/api/programs", program_routes())
        .with_state(state);

    let listner = TcpListener::bind("127.0.0.1:4000").await.unwrap();

    println!("Server running on http://localhost:{}", port);

    axum::serve(listner, app).await.unwrap();
}
