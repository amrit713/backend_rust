use crate::lesson::entity::Lesson;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait LessonRepo: Send + Sync {
    async fn create(&self, lesson: Lesson) -> anyhow::Result<Lesson>;

    async fn find_all(&self) -> anyhow::Result<Vec<Lesson>>;

    async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<Lesson>>;

    async fn update(&self, lesson: Lesson) -> anyhow::Result<Lesson>;

    async fn delete(&self, id: Uuid) -> anyhow::Result<()>;
}
