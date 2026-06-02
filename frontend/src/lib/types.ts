// frontend/src/lib/types.ts
export interface Tag {
  id: number
  slug: string
  name: string
}

export interface Category {
  id: number
  slug: string
  name: string
  description: string
}

export interface Article {
  id: number
  slug: string
  title: string
  excerpt: string
  body: string
  author_name: string
  status: 'draft' | 'published' | 'archived'
  category_id: number | null
  published_at: string | null
  created_at: string
  updated_at: string
}

export interface ArticleWithTags {
  article: Article
  tags: Tag[]
  category: Category | null
}

export interface PaginatedArticles {
  items: ArticleWithTags[]
  total: number
  page: number
  limit: number
}

export interface Page {
  id: number
  slug: string
  title: string
  body: string
  status: 'draft' | 'published'
  created_at: string
  updated_at: string
}

export interface Setting {
  key: string
  value: string
}
