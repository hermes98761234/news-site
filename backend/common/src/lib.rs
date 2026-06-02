use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Article {
    pub id: i64,
    pub slug: String,
    pub title: String,
    pub excerpt: String,
    pub body: String,
    pub author_name: String,
    pub status: String,
    pub category_id: Option<i64>,
    pub published_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleWithTags {
    #[serde(flatten)]
    pub article: Article,
    pub tags: Vec<Tag>,
    pub category: Option<Category>,
}

#[derive(Debug, Deserialize)]
pub struct CreateArticle {
    pub title: String,
    pub excerpt: Option<String>,
    pub body: String,
    pub author_name: String,
    pub category_id: Option<i64>,
    pub tag_ids: Option<Vec<i64>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateArticle {
    pub title: Option<String>,
    pub excerpt: Option<String>,
    pub body: Option<String>,
    pub author_name: Option<String>,
    pub category_id: Option<i64>,
    pub tag_ids: Option<Vec<i64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Page {
    pub id: i64,
    pub slug: String,
    pub title: String,
    pub body: String,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreatePage {
    pub title: String,
    pub slug: String,
    pub body: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePage {
    pub title: Option<String>,
    pub body: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Tag {
    pub id: i64,
    pub slug: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateTag {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Category {
    pub id: i64,
    pub slug: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateCategory {
    pub name: String,
    pub slug: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCategory {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Setting {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSetting {
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct ArticleListParams {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub tag: Option<String>,
    pub category: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PaginatedArticles {
    pub items: Vec<ArticleWithTags>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
}
