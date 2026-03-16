use crate::program::entity::Program;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait ProgramRepo: Send + Sync {
    async fn create(&self, program: Program) -> anyhow::Result<Program>;

    async fn find_all(&self) -> anyhow::Result<Vec<Program>>;

    async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<Program>>;

    async fn update(&self, program: Program) -> anyhow::Result<Program>;

    async fn delete(&self, id: Uuid) -> anyhow::Result<()>;
}
