// frontend/src/app/tags/[slug]/page.tsx
import type { Metadata } from 'next'
import { notFound } from 'next/navigation'
import { api } from '@/lib/api'
import { ArticleList } from '@/components/common/ArticleList'

interface Props { params: { slug: string } }

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
  let items
  try {
    items = await api.tags.articles(params.slug)
  } catch {
    notFound()
  }
  return (
    <div>
      <h1 className="text-3xl font-serif font-bold mb-6">#{params.slug}</h1>
      <ArticleList items={items} />
    </div>
  )
}
