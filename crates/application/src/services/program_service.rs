use async_trait::async_trait;
use chrono::Utc;
use domain::category::repo::CategoryRepo;
use domain::program::entity::Program;
use domain::program::repo::ProgramRepo;
use shared::dto::program::{CreateProgramDTO, UpdateProgramDTO};
use std::sync::Arc;
use uuid::Uuid;

use crate::traits::program_trait::IProgramService;

pub struct ProgramService<P: ProgramRepo, C: CategoryRepo> {
    program_repo: Arc<P>,
    category_repo: Arc<C>,
}

impl<P: ProgramRepo, C: CategoryRepo> ProgramService<P, C> {
    pub fn new(program_repo: Arc<P>, category_repo: Arc<C>) -> Self {
        Self {
            program_repo,
            category_repo,
        }
    }
}

#[async_trait]
impl<P, C> IProgramService for ProgramService<P, C>
where
    P: ProgramRepo + Sync + Send + 'static,
    C: CategoryRepo + Sync + Send + 'static,
{
    // !Create
    async fn create_program(&self, req: CreateProgramDTO) -> anyhow::Result<Program> {
        let category_exists = self
            .category_repo
            .find_by_id(req.category_id)
            .await?
            .is_some();

        if !category_exists {
            return Err(anyhow::anyhow!(
                "Validation Failed: Category ID {} does not exist",
                req.category_id
            ));
        }

        if req.duration < 5 {
            return Err(anyhow::anyhow!("Programs must be at least 5 minutes long"));
        }

        let program = Program::new(
            req.title,
            req.description,
            req.duration,
            req.category_id,
            req.difficulty,
        );

        self.program_repo.create(program).await
    }

    // !Get All
    async fn get_programs(&self) -> anyhow::Result<Vec<Program>> {
        self.program_repo.find_all().await
    }

    // !Get One
    async fn get_program(&self, id: Uuid) -> anyhow::Result<Program> {
        let result = self.program_repo.find_by_id(id).await?;

        match result {
            Some(program) => Ok(program),
            None => Err(anyhow::anyhow!("Category with ID {} not found", id)),
        }
    }

    // !Update
    async fn update_program(&self, id: Uuid, req: UpdateProgramDTO) -> anyhow::Result<Program> {
        let mut program = self
            .program_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Program not found"))?;

        if let Some(title) = req.title {
            program.title = title;
        }

        if req.description.is_some() {
            program.description = req.description;
        }

        if let Some(duration) = req.duration {
            program.duration = duration;
        }

        if let Some(difficulty) = req.difficulty {
            program.difficulty = difficulty;
        }

        program.updated_at = Utc::now();

        self.program_repo.update(program).await
    }

    // ! Delete
    async fn delete_program(&self, id: Uuid) -> anyhow::Result<()> {
        let existing = self.program_repo.find_by_id(id).await?;

        if existing.is_none() {
            return Err(anyhow::anyhow!("Category not found"));
        }

        self.program_repo.delete(id).await?;
        Ok(())
    }
}
