use crate::error::AppError;
use common::{CreateTag, Tag};
use slug::slugify;
use sqlx::SqlitePool;

pub async fn create(pool: &SqlitePool, input: CreateTag) -> Result<Tag, AppError> {
    let slug = slugify(&input.name);
    Ok(sqlx::query_as::<_, Tag>(
        "INSERT INTO tags (slug, name) VALUES (?, ?) RETURNING *"
    )
    .bind(&slug)
    .bind(&input.name)
    .fetch_one(pool)
    .await?)
}

pub async fn list(pool: &SqlitePool) -> Result<Vec<Tag>, AppError> {
    Ok(sqlx::query_as::<_, Tag>("SELECT * FROM tags ORDER BY name").fetch_all(pool).await?)
}

pub async fn delete(pool: &SqlitePool, slug: &str) -> Result<(), AppError> {
    let rows = sqlx::query("DELETE FROM tags WHERE slug = ?")
        .bind(slug).execute(pool).await?.rows_affected();
    if rows == 0 { return Err(AppError::NotFound(format!("tag '{slug}' not found"))); }
    Ok(())
}
