use application::traits::{category_trait::ICategoryService, program_trait::IProgramService};

use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub category_service: Arc<dyn ICategoryService>,
    pub program_service: Arc<dyn IProgramService>,
}
