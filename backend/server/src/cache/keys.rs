// backend/server/src/cache/keys.rs
pub const ARTICLES_LIST: &str = "articles:list";
pub const TAGS_LIST: &str = "tags:list";
pub const CATEGORIES_LIST: &str = "categories:list";
pub const HOMEPAGE_FEED: &str = "feed:homepage";

pub fn article_slug(slug: &str) -> String {
    format!("articles:slug:{slug}")
}
pub fn page_slug(slug: &str) -> String {
    format!("pages:slug:{slug}")
}
pub fn tag_articles(slug: &str) -> String {
    format!("tags:{slug}:articles")
}
pub fn category_articles(slug: &str) -> String {
    format!("categories:{slug}:articles")
}
