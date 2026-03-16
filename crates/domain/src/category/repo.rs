use crate::category::entity::Category;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait CategoryRepo: Send + Sync {
    async fn create(&self, category: Category) -> anyhow::Result<Category>;

    async fn find_all(&self) -> anyhow::Result<Vec<Category>>;

    async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<Category>>;

    async fn update(&self, category: Category) -> anyhow::Result<Category>;

    async fn delete(&self, id: Uuid) -> anyhow::Result<()>;
}
