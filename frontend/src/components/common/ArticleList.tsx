import type { ArticleWithTags, PaginatedArticles } from '@/lib/types'
import { ArticleCard } from './ArticleCard'
import { Pagination } from './Pagination'

interface ArticleListProps {
  paginated?: PaginatedArticles
  items?: ArticleWithTags[]
  basePath?: string
}

export function ArticleList({ paginated, items, basePath = '/articles' }: ArticleListProps) {
  const data = items ?? paginated?.items ?? []
  const page = paginated?.page ?? 1
  const total = paginated?.total ?? data.length
  const limit = paginated?.limit ?? data.length

  if (data.length === 0) {
    return <p className="text-gray-500 text-center py-12">No articles found.</p>
  }

  return (
    <div>
      <div>
        {data.map(item => (
          <ArticleCard key={item.id} article={item} />
        ))}
      </div>
      {paginated && <Pagination page={page} total={total} limit={limit} basePath={basePath} />}
    </div>
  )
}
