use async_trait::async_trait;
use domain::category::entity::Category;
use domain::category::repo::CategoryRepo;
use sqlx::PgPool;
use uuid::Uuid;

pub struct SqlxCategoryRepo {
    pub pool: PgPool,
}

impl SqlxCategoryRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CategoryRepo for SqlxCategoryRepo {
    async fn create(&self, category: Category) -> anyhow::Result<Category> {
        let new_category = sqlx::query_as!(
            Category,
            r#"
            INSERT INTO categories (id, name, created_at, updated_at)
            VALUES ($1, $2, $3, $4)
            RETURNING id, name, created_at, updated_at
            "#,
            category.id,
            category.name,
            category.created_at,
            category.updated_at
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(new_category)
    }

    async fn find_all(&self) -> anyhow::Result<Vec<Category>> {
        let categories = sqlx::query_as!(
            Category,
            "SELECT id, name, created_at,updated_at FROM categories ORDER BY name ASC"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(categories)
    }

    async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<Category>> {
        let category = sqlx::query_as!(
            Category,
            "SELECT id, name, created_at, updated_at FROM categories WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(category)
    }

    async fn update(&self, category: Category) -> anyhow::Result<Category> {
        let updated = sqlx::query_as!(
            Category,
            r#"
            UPDATE categories 
            SET name = $1, updated_at = NOW() 
            WHERE id = $2 
            RETURNING id, name, created_at, updated_at
            "#,
            category.name,
            category.id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(updated)
    }

    async fn delete(&self, id: Uuid) -> anyhow::Result<()> {
        let result = sqlx::query!("DELETE FROM categories WHERE id = $1", id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(anyhow::anyhow!("Category not found"));
        }

        Ok(())
    }
}
