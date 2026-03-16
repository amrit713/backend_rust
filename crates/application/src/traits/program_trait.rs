use async_trait::async_trait;
use domain::program::entity::Program;
use shared::dto::program::{CreateProgramDTO, UpdateProgramDTO};
use uuid::Uuid;

#[async_trait]
pub trait IProgramService: Send + Sync {
    async fn create_program(&self, program: CreateProgramDTO) -> anyhow::Result<Program>;

    async fn get_programs(&self) -> anyhow::Result<Vec<Program>>;

    async fn get_program(&self, id: Uuid) -> anyhow::Result<Program>;

    async fn update_program(&self, id: Uuid, req: UpdateProgramDTO) -> anyhow::Result<Program>;

    async fn delete_program(&self, id: Uuid) -> anyhow::Result<()>;
}
