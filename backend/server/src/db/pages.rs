use crate::error::AppError;
use common::{CreatePage, Page, UpdatePage};
use sqlx::SqlitePool;

pub async fn create(pool: &SqlitePool, input: CreatePage) -> Result<Page, AppError> {
    let existing = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM pages WHERE slug = ?")
        .bind(&input.slug)
        .fetch_one(pool)
        .await?;
    if existing > 0 {
        return Err(AppError::Conflict(format!("slug '{}' already exists", input.slug)));
    }
    Ok(sqlx::query_as::<_, Page>(
        "INSERT INTO pages (slug, title, body) VALUES (?, ?, ?) RETURNING *"
    )
    .bind(&input.slug)
    .bind(&input.title)
    .bind(input.body.unwrap_or_default())
    .fetch_one(pool)
    .await?)
}

pub async fn get_by_id(pool: &SqlitePool, id: i64) -> Result<Page, AppError> {
    sqlx::query_as::<_, Page>("SELECT * FROM pages WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("page {id} not found")))
}

pub async fn get_by_slug(pool: &SqlitePool, slug: &str) -> Result<Page, AppError> {
    sqlx::query_as::<_, Page>("SELECT * FROM pages WHERE slug = ?")
        .bind(slug)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("page '{slug}' not found")))
}

pub async fn list(pool: &SqlitePool) -> Result<Vec<Page>, AppError> {
    Ok(sqlx::query_as::<_, Page>("SELECT * FROM pages ORDER BY created_at DESC")
        .fetch_all(pool)
        .await?)
}

pub async fn update(pool: &SqlitePool, id: i64, input: UpdatePage) -> Result<Page, AppError> {
    sqlx::query_as::<_, Page>(
        "UPDATE pages SET title = COALESCE(?, title), body = COALESCE(?, body),
         updated_at = datetime('now') WHERE id = ? RETURNING *"
    )
    .bind(input.title)
    .bind(input.body)
    .bind(id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("page {id} not found")))
}

pub async fn publish(pool: &SqlitePool, id: i64) -> Result<Page, AppError> {
    sqlx::query_as::<_, Page>(
        "UPDATE pages SET status = 'published', updated_at = datetime('now') WHERE id = ? RETURNING *"
    )
    .bind(id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("page {id} not found")))
}

pub async fn delete(pool: &SqlitePool, id: i64) -> Result<(), AppError> {
    let rows = sqlx::query("DELETE FROM pages WHERE id = ?")
        .bind(id).execute(pool).await?.rows_affected();
    if rows == 0 { return Err(AppError::NotFound(format!("page {id} not found"))); }
    Ok(())
}
