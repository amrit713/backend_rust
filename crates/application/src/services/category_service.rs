use async_trait::async_trait;
use domain::category::entity::Category;
use domain::category::repo::CategoryRepo;
use std::sync::Arc;
use uuid::Uuid;

use crate::traits::category_trait::ICategoryService;

pub struct CategoryService<R: CategoryRepo> {
    repo: Arc<R>,
}

impl<R: CategoryRepo> CategoryService<R> {
    pub fn new(repo: Arc<R>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl<R> ICategoryService for CategoryService<R>
where
    R: CategoryRepo + Sync + Send + 'static,
{
    //  !Create
    async fn create_category(&self, name: String) -> anyhow::Result<Category> {
        if name.trim().is_empty() {
            return Err(anyhow::anyhow!("Category name cannot be empty"));
        }

        let new_category = Category::new(name);
        self.repo.create(new_category).await
    }

    // !Get All
    async fn get_categories(&self) -> anyhow::Result<Vec<Category>> {
        self.repo.find_all().await
    }

    // !Get One
    async fn get_category(&self, id: Uuid) -> anyhow::Result<Category> {
        let result = self.repo.find_by_id(id).await?;

        match result {
            Some(category) => Ok(category),
            None => Err(anyhow::anyhow!("Category with ID {} not found", id)),
        }
    }

    // !Update
    async fn update_category(&self, id: Uuid, name: String) -> anyhow::Result<Category> {
        if name.trim().is_empty() {
            return Err(anyhow::anyhow!("New category name cannot be empty"));
        }

        let existing = self.repo.find_by_id(id).await?;

        if existing.is_none() {
            return Err(anyhow::anyhow!(
                "Cannot update: Category with ID {} not found",
                id
            ));
        }

        let mut category = existing.unwrap();
        category.name = name.trim().to_string();

        self.repo.update(category).await
    }

    // !Delete
    async fn delete_category(&self, id: Uuid) -> anyhow::Result<()> {
        let existing = self.repo.find_by_id(id).await?;
        if existing.is_none() {
            return Err(anyhow::anyhow!("Category not found"));
        }

        self.repo.delete(id).await?;

        Ok(())
    }
}
