// frontend/src/lib/api.ts
import type { ArticleWithTags, Category, Page, PaginatedArticles, Setting, Tag } from './types'

const API_URL = (process.env.NEXT_PUBLIC_API_URL || 'http://localhost:3000').replace(/\/+$/, '')

async function fetchApi<T>(path: string): Promise<T> {
  const res = await fetch(`${API_URL}/api${path}`, {
    next: { revalidate: 300 },
    signal: AbortSignal.timeout(5000),
  })
  if (!res.ok) throw new Error(`API error ${res.status}: ${path}`)
  return res.json()
}

export const api = {
  articles: {
    list: (page = 1, limit = 20, tag?: string, category?: string) => {
      const params = new URLSearchParams({ page: String(page), limit: String(limit) })
      if (tag) params.set('tag', tag)
      if (category) params.set('category', category)
      return fetchApi<PaginatedArticles>(`/articles?${params}`)
    },
    get: (slug: string) => fetchApi<ArticleWithTags>(`/articles/${slug}`),
    slugs: async () => {
      const data = await fetchApi<PaginatedArticles>('/articles?limit=1000')
      return data.items.map(i => i.slug)
    },
  },
  pages: {
    list: () => fetchApi<Page[]>('/pages'),
    get: (slug: string) => fetchApi<Page>(`/pages/${slug}`),
  },
  tags: {
    list: () => fetchApi<Tag[]>('/tags'),
    articles: (slug: string) => fetchApi<ArticleWithTags[]>(`/tags/${slug}/articles`),
  },
  categories: {
    list: () => fetchApi<Category[]>('/categories'),
    articles: (slug: string, category: string) =>
      fetchApi<PaginatedArticles>(`/articles?category=${category}`),
  },
  settings: {
    get: async () => {
      const settings = await fetchApi<Setting[]>('/settings')
      return Object.fromEntries(settings.map(s => [s.key, s.value])) as Record<string, string>
    },
  },
}
