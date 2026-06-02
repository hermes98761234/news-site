use common::db::articles::*;
use common::*;
use sqlx::SqlitePool;

pub async fn list_articles(
    pool: &SqlitePool,
    params: &ArticleListParams,
) -> anyhow::Result<PaginatedArticles> {
    let page = params.page.unwrap_or(1).max(1);
    let limit = params.limit.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * limit;

    let mut query = String::from(
        "SELECT a.* FROM articles a WHERE a.status = 'published'",
    );
    let mut count_query = String::from(
        "SELECT COUNT(*) FROM articles a WHERE a.status = 'published'",
    );
    let mut args: Vec<String> = Vec::new();

    if let Some(ref tag) = params.tag {
        let join = format!(
            " INNER JOIN article_tags at ON at.article_id = a.id INNER JOIN tags t ON t.id = at.tag_id AND t.slug = '{}'",
            tag
        );
        query.push_str(&join);
        count_query.push_str(&join);
    }

    if let Some(ref category) = params.category {
        let join = format!(
            " INNER JOIN categories c ON c.id = a.category_id AND c.slug = '{}'",
            category
        );
        query.push_str(&join);
        count_query.push_str(&join);
    }

    query.push_str(" ORDER BY a.published_at DESC LIMIT ? OFFSET ?");

    let total: i64 = sqlx::query_scalar(&count_query)
        .fetch_one(pool)
        .await?;

    let articles: Vec<Article> = sqlx::query_as::<_, Article>(&query)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;

    // Fetch tags for each article
    let mut items = Vec::new();
    for article in articles {
        let tags = get_article_tags(pool, article.id).await?;
        let category = match article.category_id {
            Some(cid) => get_category(pool, cid).await?,
            None => None,
        };
        items.push(ArticleWithTags {
            article,
            tags,
            category,
        });
    }

    Ok(PaginatedArticles {
        items,
        total,
        page,
        limit,
    })
}

pub async fn get_article_by_slug(
    pool: &SqlitePool,
    slug: &str,
) -> anyhow::Result<Option<ArticleWithTags>> {
    let article: Option<Article> = sqlx::query_as::<_, Article>(
        "SELECT * FROM articles WHERE slug = ?",
    )
    .bind(slug)
    .fetch_optional(pool)
    .await?;

    match article {
        Some(article) => {
            let tags = get_article_tags(pool, article.id).await?;
            let category = match article.category_id {
                Some(cid) => get_category(pool, cid).await?,
                None => None,
            };
            Ok(Some(ArticleWithTags {
                article,
                tags,
                category,
            }))
        }
        None => Ok(None),
    }
}

pub async fn create_article(
    pool: &SqlitePool,
    input: &CreateArticle,
) -> anyhow::Result<Article> {
    let slug = slug::slugify(&input.title);
    let excerpt = input.excerpt.clone().unwrap_or_default();

    let id = sqlx::query(
        "INSERT INTO articles (slug, title, excerpt, body, author_name, category_id, status)
         VALUES (?, ?, ?, ?, ?, ?, 'draft')",
    )
    .bind(&slug)
    .bind(&input.title)
    .bind(&excerpt)
    .bind(&input.body)
    .bind(&input.author_name)
    .bind(input.category_id)
    .execute(pool)
    .await?
    .last_insert_rowid();

    if let Some(ref tag_ids) = input.tag_ids {
        for tag_id in tag_ids {
            sqlx::query("INSERT OR IGNORE INTO article_tags (article_id, tag_id) VALUES (?, ?)")
                .bind(id)
                .bind(tag_id)
                .execute(pool)
                .await?;
        }
    }

    let article: Article = sqlx::query_as::<_, Article>("SELECT * FROM articles WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await?;

    Ok(article)
}

pub async fn update_article(
    pool: &SqlitePool,
    id: i64,
    input: &UpdateArticle,
) -> anyhow::Result<Option<Article>> {
    let existing: Option<Article> =
        sqlx::query_as::<_, Article>("SELECT * FROM articles WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await?;

    if existing.is_none() {
        return Ok(None);
    }

    let mut sets: Vec<String> = Vec::new();

    if input.title.is_some() {
        sets.push("title = ?".to_string());
    }
    if input.excerpt.is_some() {
        sets.push("excerpt = ?".to_string());
    }
    if input.body.is_some() {
        sets.push("body = ?".to_string());
    }
    if input.author_name.is_some() {
        sets.push("author_name = ?".to_string());
    }
    if input.category_id.is_some() {
        sets.push("category_id = ?".to_string());
    }

    if !sets.is_empty() {
        sets.push("updated_at = datetime('now')".to_string());
        let set_clause = sets.join(", ");
        let sql = format!("UPDATE articles SET {} WHERE id = ?", set_clause);

        let mut q = sqlx::query(&sql);
        if let Some(ref title) = input.title {
            q = q.bind(title);
        }
        if let Some(ref excerpt) = input.excerpt {
            q = q.bind(excerpt);
        }
        if let Some(ref body) = input.body {
            q = q.bind(body);
        }
        if let Some(ref author) = input.author_name {
            q = q.bind(author);
        }
        if let Some(ref cat_id) = input.category_id {
            q = q.bind(cat_id);
        }
        q.bind(id).execute(pool).await?;
    }

    if let Some(ref tag_ids) = input.tag_ids {
        sqlx::query("DELETE FROM article_tags WHERE article_id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        for tag_id in tag_ids {
            sqlx::query("INSERT OR IGNORE INTO article_tags (article_id, tag_id) VALUES (?, ?)")
                .bind(id)
                .bind(tag_id)
                .execute(pool)
                .await?;
        }
    }

    let article: Option<Article> =
        sqlx::query_as::<_, Article>("SELECT * FROM articles WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await?;

    Ok(article)
}

pub async fn delete_article(pool: &SqlitePool, id: i64) -> anyhow::Result<bool> {
    let result = sqlx::query("DELETE FROM articles WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected() > 0)
}

pub async fn publish_article(pool: &SqlitePool, id: i64) -> anyhow::Result<Option<Article>> {
    let existing: Option<Article> =
        sqlx::query_as::<_, Article>("SELECT * FROM articles WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await?;

    if existing.is_none() {
        return Ok(None);
    }

    sqlx::query(
        "UPDATE articles SET status = 'published', published_at = datetime('now'), updated_at = datetime('now') WHERE id = ?",
    )
    .bind(id)
    .execute(pool)
    .await?;

    let article: Option<Article> =
        sqlx::query_as::<_, Article>("SELECT * FROM articles WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await?;

    Ok(article)
}

pub async fn archive_article(pool: &SqlitePool, id: i64) -> anyhow::Result<Option<Article>> {
    let existing: Option<Article> =
        sqlx::query_as::<_, Article>("SELECT * FROM articles WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await?;

    if existing.is_none() {
        return Ok(None);
    }

    sqlx::query(
        "UPDATE articles SET status = 'archived', updated_at = datetime('now') WHERE id = ?",
    )
    .bind(id)
    .execute(pool)
    .await?;

    let article: Option<Article> =
        sqlx::query_as::<_, Article>("SELECT * FROM articles WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await?;

    Ok(article)
}

// --- helpers ---

async fn get_article_tags(pool: &SqlitePool, article_id: i64) -> anyhow::Result<Vec<Tag>> {
    let tags = sqlx::query_as::<_, Tag>(
        "SELECT t.* FROM tags t
         INNER JOIN article_tags at ON at.tag_id = t.id
         WHERE at.article_id = ?",
    )
    .bind(article_id)
    .fetch_all(pool)
    .await?;
    Ok(tags)
}

async fn get_category(pool: &SqlitePool, category_id: i64) -> anyhow::Result<Option<Category>> {
    let cat = sqlx::query_as::<_, Category>("SELECT * FROM categories WHERE id = ?")
        .bind(category_id)
        .fetch_optional(pool)
        .await?;
    Ok(cat)
}
