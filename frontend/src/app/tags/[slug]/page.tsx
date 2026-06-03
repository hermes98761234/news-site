// frontend/src/app/tags/[slug]/page.tsx
import type { Metadata } from 'next'
import { api } from '@/lib/api'
import { ArticleList } from '@/components/common/ArticleList'
import type { ArticleWithTags } from '@/lib/types'

interface Props { params: { slug: string } }

export const dynamicParams = false

export async function generateStaticParams() {
  try {
    const tags = await api.tags.list()
    return tags.map(t => ({ slug: t.slug }))
  } catch {
    return []
  }
}

export async function generateMetadata({ params }: Props): Promise<Metadata> {
  return { title: `Tag: ${params.slug}` }
}

export default async function TagPage({ params }: Props) {
  let items: ArticleWithTags[] = []
  try {
    items = await api.tags.articles(params.slug)
  } catch {
    // API unavailable or tag not found — render empty list
  }
  return (
    <div>
      <h1 className="text-3xl font-serif font-bold mb-6">#{params.slug}</h1>
      <ArticleList items={items} />
    </div>
  )
}
