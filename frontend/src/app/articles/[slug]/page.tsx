// frontend/src/app/articles/[slug]/page.tsx
import { api } from '@/lib/api'
import { ArticleBody } from '@/components/common/ArticleBody'
import { TagBadge } from '@/components/common/TagBadge'
import { CategoryBadge } from '@/components/common/CategoryBadge'
import type { ArticleWithTags } from '@/lib/types'

interface Props {
  params: { slug: string }
}

export async function generateStaticParams() {
  try {
    const slugs = await api.articles.slugs()
    return slugs.map((slug: string) => ({ slug }))
  } catch {
    return []
  }
}

function formatDate(dateStr: string): string {
  return new Date(dateStr).toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
  })
}

export default async function ArticlePage({ params }: Props) {
  let data: ArticleWithTags
  try {
    data = await api.articles.get(params.slug)
  } catch {
    data = { id: 0, title: params.slug, body: '', slug: params.slug, excerpt: '', status: 'published' as const, created_at: '', updated_at: '', author_name: '', published_at: null, category_id: null, tags: [], category: null }
  }
  const { tags, category, ...a } = data

  return (
    <article>
      <header className="mb-8">
        <div className="flex items-center gap-2 mb-2 text-sm text-gray-500">
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
        <h1 className="text-4xl font-serif font-bold text-gray-900 mb-4">{a.title}</h1>
        {a.author_name && (
          <p className="text-sm text-gray-500">by {a.author_name}</p>
        )}
        {tags.length > 0 && (
          <div className="flex items-center gap-2 mt-4 flex-wrap">
            {tags.map((tag) => (
              <TagBadge key={tag.id} tag={tag} />
            ))}
          </div>
        )}
      </header>
      <ArticleBody>{a.body}</ArticleBody>
    </article>
  )
}
