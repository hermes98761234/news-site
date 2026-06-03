// frontend/src/app/categories/[slug]/page.tsx
import type { Metadata } from 'next'
import { api } from '@/lib/api'
import { ArticleList } from '@/components/common/ArticleList'
import type { PaginatedArticles } from '@/lib/types'

interface Props { params: { slug: string } }

export const dynamicParams = false

export async function generateStaticParams() {
  try {
    const cats = await api.categories.list()
    return cats.map(c => ({ slug: c.slug }))
  } catch {
    return []
  }
}

export async function generateMetadata({ params }: Props): Promise<Metadata> {
  return { title: params.slug }
}

export default async function CategoryPage({ params }: Props) {
  let data: PaginatedArticles | undefined
  try {
    data = await api.categories.articles(params.slug, params.slug)
  } catch {
    // API unavailable or category not found — render empty list
  }
  return (
    <div>
      <h1 className="text-3xl font-serif font-bold mb-6 capitalize">{params.slug}</h1>
      <ArticleList paginated={data} />
    </div>
  )
}
