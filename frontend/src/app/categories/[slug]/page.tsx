// frontend/src/app/categories/[slug]/page.tsx
import type { Metadata } from 'next'
import { notFound } from 'next/navigation'
import { api } from '@/lib/api'
import { ArticleList } from '@/components/common/ArticleList'

interface Props { params: { slug: string } }

export async function generateStaticParams() {
  const cats = await api.categories.list()
  return cats.map(c => ({ slug: c.slug }))
}

export async function generateMetadata({ params }: Props): Promise<Metadata> {
  return { title: params.slug }
}

export default async function CategoryPage({ params }: Props) {
  let data
  try {
    data = await api.categories.articles(params.slug, params.slug)
  } catch {
    notFound()
  }
  return (
    <div>
      <h1 className="text-3xl font-serif font-bold mb-6 capitalize">{params.slug}</h1>
      <ArticleList paginated={data} />
    </div>
  )
}
