use crate::chapter::entity::Chapter;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait ChapterRepo: Send + Sync {
    async fn create(&self, chapter: Chapter) -> anyhow::Result<Chapter>;

    async fn find_all(&self) -> anyhow::Result<Vec<Chapter>>;

    async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<Chapter>>;

    async fn update(&self, chapter: Chapter) -> anyhow::Result<Chapter>;

    async fn delete(&self, id: Uuid) -> anyhow::Result<()>;
}
