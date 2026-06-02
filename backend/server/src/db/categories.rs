use crate::error::AppError;
use common::{Category, CreateCategory, UpdateCategory};
use slug::slugify;
use sqlx::SqlitePool;

pub async fn create(pool: &SqlitePool, input: CreateCategory) -> Result<Category, AppError> {
    let slug = input.slug.unwrap_or_else(|| slugify(&input.name));
    Ok(sqlx::query_as::<_, Category>(
        "INSERT INTO categories (slug, name, description) VALUES (?, ?, ?) RETURNING *"
    )
    .bind(&slug)
    .bind(&input.name)
    .bind(input.description.unwrap_or_default())
    .fetch_one(pool)
    .await?)
}

pub async fn list(pool: &SqlitePool) -> Result<Vec<Category>, AppError> {
    Ok(sqlx::query_as::<_, Category>("SELECT * FROM categories ORDER BY name").fetch_all(pool).await?)
}

pub async fn get_by_slug(pool: &SqlitePool, slug: &str) -> Result<Category, AppError> {
    sqlx::query_as::<_, Category>("SELECT * FROM categories WHERE slug = ?")
        .bind(slug)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("category '{slug}' not found")))
}

pub async fn update(pool: &SqlitePool, id: i64, input: UpdateCategory) -> Result<Category, AppError> {
    sqlx::query_as::<_, Category>(
        "UPDATE categories SET name = COALESCE(?, name), description = COALESCE(?, description),
         updated_at = datetime('now') WHERE id = ? RETURNING *"
    )
    .bind(input.name)
    .bind(input.description)
    .bind(id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("category {id} not found")))
}

pub async fn delete(pool: &SqlitePool, slug: &str) -> Result<(), AppError> {
    let rows = sqlx::query("DELETE FROM categories WHERE slug = ?")
        .bind(slug).execute(pool).await?.rows_affected();
    if rows == 0 { return Err(AppError::NotFound(format!("category '{slug}' not found"))); }
    Ok(())
}
