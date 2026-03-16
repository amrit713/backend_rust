use async_trait::async_trait;
use domain::category::entity::Category;
use uuid::Uuid;

#[async_trait]
pub trait ICategoryService: Send + Sync {
    async fn create_category(&self, name: String) -> anyhow::Result<Category>;

    async fn get_categories(&self) -> anyhow::Result<Vec<Category>>;

    async fn get_category(&self, id: Uuid) -> anyhow::Result<Category>;

    async fn update_category(&self, id: Uuid, name: String) -> anyhow::Result<Category>;

    async fn delete_category(&self, id: Uuid) -> anyhow::Result<()>;
}
