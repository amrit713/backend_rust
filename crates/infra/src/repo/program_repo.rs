use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use domain::program::entity::{Program,Difficulty};
use domain::program::repo::ProgramRepo;

pub struct SqlxProgramRepo {
    pub pool: PgPool,
}

impl SqlxProgramRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}


#[async_trait]
impl ProgramRepo for SqlxProgramRepo {
    async fn create(&self, program: Program) -> anyhow::Result<Program> {
       sqlx::query!(
            r#"
            INSERT INTO programs (id, title, description, duration, difficulty, category_id, created_at, updated_at) 
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            program.id, 
            program.title, 
            program.description, 
            program.duration as i16, 
            program.difficulty.as_ref(), // Pass it as a string slice
            program.category_id, 
            program.created_at, 
            program.updated_at
        )
        .execute(&self.pool)
        .await?;

        Ok(program) 
    }

    async fn find_all(&self) -> anyhow::Result<Vec<Program>> {
    let programs = sqlx::query_as!(
        Program, 
        r#"
        SELECT 
            id, 
            title, 
            description, 
            duration, 
            difficulty AS "difficulty: Difficulty", -- This tells SQLx to use your Enum's Decode trait
            category_id, 
            created_at, 
            updated_at 
        FROM programs
        "#
    )
    .fetch_all(&self.pool)
    .await?;

    Ok(programs)
}

async fn find_by_id(&self, id:Uuid) -> anyhow::Result<Option<Program>>{
let program = sqlx::query_as!(
        Program,
        r#"
        SELECT 
            id, 
            title, 
            description, 
            duration, 
            difficulty AS "difficulty: Difficulty", 
            category_id, 
            created_at, 
            updated_at
        FROM programs 
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(&self.pool)
    .await?;

    Ok(program)
}

 async fn update(&self, program: Program) -> anyhow::Result<Program>{
let updated_program = sqlx::query_as!(
            Program,
            r#"
            UPDATE programs 
            SET title = $1, description = $2, duration = $3, difficulty = $4, category_id = $5, updated_at = NOW()
            WHERE id = $6
            RETURNING id, title, description, duration, difficulty AS "difficulty: Difficulty", category_id, created_at, updated_at
            "#,
            program.title,
            program.description,
            program.duration as i16,
            program.difficulty as Difficulty,
            program.category_id,
            program.id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(updated_program)
    }
 

async fn delete(&self, id: Uuid) -> anyhow::Result<()> {
        let result = sqlx::query!(
            "DELETE FROM programs WHERE id = $1",
            id
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(anyhow::anyhow!("Program with ID {} not found", id));
        }

        Ok(())
    }




}