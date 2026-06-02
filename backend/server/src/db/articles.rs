use crate::error::AppError;
use common::{Article, ArticleListParams, ArticleWithTags, Category, CreateArticle, Tag, UpdateArticle};
use sqlx::SqlitePool;
use slug::slugify;

pub async fn create(pool: &SqlitePool, input: CreateArticle) -> Result<Article, AppError> {
    let slug = slugify(&input.title);
    let existing = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM articles WHERE slug = ?"
    )
    .bind(&slug)
    .fetch_one(pool)
    .await?;
    if existing > 0 {
        return Err(AppError::Conflict(format!("slug '{slug}' already exists")));
    }
    let article = sqlx::query_as::<_, Article>(
        "INSERT INTO articles (slug, title, excerpt, body, author_name, category_id)
         VALUES (?, ?, ?, ?, ?, ?)
         RETURNING *"
    )
    .bind(&slug)
    .bind(&input.title)
    .bind(input.excerpt.unwrap_or_default())
    .bind(&input.body)
    .bind(&input.author_name)
    .bind(input.category_id)
    .fetch_one(pool)
    .await?;
    if let Some(tag_ids) = input.tag_ids {
        for tag_id in tag_ids {
            sqlx::query("INSERT OR IGNORE INTO article_tags (article_id, tag_id) VALUES (?, ?)")
                .bind(article.id)
                .bind(tag_id)
                .execute(pool)
                .await?;
        }
    }
    Ok(article)
}

pub async fn get_by_id(pool: &SqlitePool, id: i64) -> Result<ArticleWithTags, AppError> {
    let article = sqlx::query_as::<_, Article>(
        "SELECT * FROM articles WHERE id = ?"
    )
    .bind(id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("article {id} not found")))?;

    let tags = sqlx::query_as::<_, Tag>(
        "SELECT t.* FROM tags t
         JOIN article_tags at ON at.tag_id = t.id
         WHERE at.article_id = ?"
    )
    .bind(article.id)
    .fetch_all(pool)
    .await?;

    let category = if let Some(cat_id) = article.category_id {
        sqlx::query_as::<_, Category>("SELECT * FROM categories WHERE id = ?")
            .bind(cat_id)
            .fetch_optional(pool)
            .await?
    } else {
        None
    };

    Ok(ArticleWithTags { article, tags, category })
}

pub async fn get_by_slug(pool: &SqlitePool, slug: &str) -> Result<ArticleWithTags, AppError> {
    let article = sqlx::query_as::<_, Article>(
        "SELECT * FROM articles WHERE slug = ?"
    )
    .bind(slug)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("article '{slug}' not found")))?;

    let tags = sqlx::query_as::<_, Tag>(
        "SELECT t.* FROM tags t
         JOIN article_tags at ON at.tag_id = t.id
         WHERE at.article_id = ?"
    )
    .bind(article.id)
    .fetch_all(pool)
    .await?;

    let category = if let Some(cat_id) = article.category_id {
        sqlx::query_as::<_, Category>("SELECT * FROM categories WHERE id = ?")
            .bind(cat_id)
            .fetch_optional(pool)
            .await?
    } else {
        None
    };

    Ok(ArticleWithTags { article, tags, category })
}

pub async fn list(
    pool: &SqlitePool,
    params: &ArticleListParams,
    status: Option<&str>,
) -> Result<(Vec<ArticleWithTags>, i64), AppError> {
    let page = params.page.unwrap_or(1).max(1);
    let limit = params.limit.unwrap_or(20).min(100);
    let offset = (page - 1) * limit;
    let status_filter = status.unwrap_or("published");

    let articles = sqlx::query_as::<_, Article>(
        "SELECT a.* FROM articles a
         LEFT JOIN categories c ON c.id = a.category_id
         WHERE a.status = ?
           AND (? IS NULL OR c.slug = ?)
         ORDER BY a.published_at DESC, a.created_at DESC
         LIMIT ? OFFSET ?"
    )
    .bind(status_filter)
    .bind(&params.category)
    .bind(&params.category)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    let total = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM articles a
         LEFT JOIN categories c ON c.id = a.category_id
         WHERE a.status = ?
           AND (? IS NULL OR c.slug = ?)"
    )
    .bind(status_filter)
    .bind(&params.category)
    .bind(&params.category)
    .fetch_one(pool)
    .await?;

    let mut result = Vec::new();
    for article in articles {
        let tags = sqlx::query_as::<_, Tag>(
            "SELECT t.* FROM tags t JOIN article_tags at ON at.tag_id = t.id WHERE at.article_id = ?"
        )
        .bind(article.id)
        .fetch_all(pool)
        .await?;

        if let Some(ref tag_slug) = params.tag {
            if !tags.iter().any(|t| &t.slug == tag_slug) {
                continue;
            }
        }

        let category = if let Some(cat_id) = article.category_id {
            sqlx::query_as::<_, Category>("SELECT * FROM categories WHERE id = ?")
                .bind(cat_id)
                .fetch_optional(pool)
                .await?
        } else {
            None
        };

        result.push(ArticleWithTags { article, tags, category });
    }

    Ok((result, total))
}

pub async fn update(pool: &SqlitePool, id: i64, input: UpdateArticle) -> Result<Article, AppError> {
    let article = sqlx::query_as::<_, Article>("SELECT * FROM articles WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("article {id} not found")))?;

    let article = sqlx::query_as::<_, Article>(
        "UPDATE articles SET
           title       = COALESCE(?, title),
           excerpt     = COALESCE(?, excerpt),
           body        = COALESCE(?, body),
           author_name = COALESCE(?, author_name),
           category_id = COALESCE(?, category_id),
           updated_at  = datetime('now')
         WHERE id = ?
         RETURNING *"
    )
    .bind(input.title)
    .bind(input.excerpt)
    .bind(input.body)
    .bind(input.author_name)
    .bind(input.category_id)
    .bind(article.id)
    .fetch_one(pool)
    .await?;

    if let Some(tag_ids) = input.tag_ids {
        sqlx::query("DELETE FROM article_tags WHERE article_id = ?")
            .bind(article.id)
            .execute(pool)
            .await?;
        for tag_id in tag_ids {
            sqlx::query("INSERT OR IGNORE INTO article_tags (article_id, tag_id) VALUES (?, ?)")
                .bind(article.id)
                .bind(tag_id)
                .execute(pool)
                .await?;
        }
    }

    Ok(article)
}

pub async fn publish(pool: &SqlitePool, id: i64) -> Result<Article, AppError> {
    sqlx::query_as::<_, Article>(
        "UPDATE articles SET status = 'published', published_at = datetime('now'), updated_at = datetime('now')
         WHERE id = ? RETURNING *"
    )
    .bind(id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("article {id} not found")))
}

pub async fn archive(pool: &SqlitePool, id: i64) -> Result<Article, AppError> {
    sqlx::query_as::<_, Article>(
        "UPDATE articles SET status = 'archived', updated_at = datetime('now')
         WHERE id = ? RETURNING *"
    )
    .bind(id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("article {id} not found")))
}

pub async fn delete(pool: &SqlitePool, id: i64) -> Result<(), AppError> {
    let rows = sqlx::query("DELETE FROM articles WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?
        .rows_affected();
    if rows == 0 {
        return Err(AppError::NotFound(format!("article {id} not found")));
    }
    Ok(())
}
