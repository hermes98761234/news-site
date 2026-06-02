// frontend/src/app/articles/page.tsx
import { api } from '@/lib/api'
import { ArticleList } from '@/components/common/ArticleList'

export default async function ArticlesPage() {
  let articles
  try {
    articles = await api.articles.list(1, 100)
  } catch {
    articles = { items: [], total: 0, page: 1, limit: 100 }
  }

  return (
    <div>
      <h1 className="text-3xl font-serif font-bold text-gray-900 mb-8">Articles</h1>
      <ArticleList paginated={articles} />
    </div>
  )
}
