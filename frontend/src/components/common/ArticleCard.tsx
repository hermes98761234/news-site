import Link from 'next/link'
import type { ArticleWithTags } from '@/lib/types'
import { TagBadge } from './TagBadge'
import { CategoryBadge } from './CategoryBadge'

interface ArticleCardProps {
  article: ArticleWithTags
}

function formatDate(dateStr: string): string {
  return new Date(dateStr).toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
  })
}

export function ArticleCard({ article }: ArticleCardProps) {
  const { tags, category, ...a } = article

  return (
    <article className="py-6 border-b border-gray-100 last:border-b-0">
      <div className="flex items-center gap-2 mb-1 text-sm text-gray-500">
        <time dateTime={a.published_at ?? a.created_at}>
          {formatDate(a.published_at ?? a.created_at)}
        </time>
        {category && (
          <>
            <span>·</span>
            <CategoryBadge category={category} />
          </>
        )}
      </div>
      <h2 className="text-xl font-serif font-bold text-gray-900 mb-2">
        <Link href={`/articles/${a.slug}`} className="hover:text-blue-600 transition-colors">
          {a.title}
        </Link>
      </h2>
      {a.excerpt && <p className="text-gray-600 mb-3 leading-relaxed">{a.excerpt}</p>}
      <div className="flex items-center gap-2 flex-wrap">
        {tags.map(tag => (
          <TagBadge key={tag.id} tag={tag} />
        ))}
        {a.author_name && <span className="text-sm text-gray-400">by {a.author_name}</span>}
      </div>
    </article>
  )
}
