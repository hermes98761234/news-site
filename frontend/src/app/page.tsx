// frontend/src/app/page.tsx
import { api } from '@/lib/api'
import { ArticleList } from '@/components/common/ArticleList'

export default async function HomePage() {
  const articles = await api.articles.list(1, 20)

  return (
    <div>
      <h1 className="text-3xl font-serif font-bold text-gray-900 mb-8">Latest Articles</h1>
      <ArticleList paginated={articles} />
    </div>
  )
}
