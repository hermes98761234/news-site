// frontend/src/app/[slug]/page.tsx
import type { Metadata } from 'next'
import { notFound } from 'next/navigation'
import { api } from '@/lib/api'
import { ArticleBody } from '@/components/common/ArticleBody'

interface Props { params: { slug: string } }

export async function generateStaticParams() {
  return []
}

export async function generateMetadata({ params }: Props): Promise<Metadata> {
  try {
    const page = await api.pages.get(params.slug)
    return { title: page.title }
  } catch {
    return {}
  }
}

export default async function StaticPage({ params }: Props) {
  let page
  try {
    page = await api.pages.get(params.slug)
  } catch {
    notFound()
  }
  return (
    <div className="max-w-2xl">
      <h1 className="text-4xl font-serif font-bold mb-6">{page.title}</h1>
      <ArticleBody>{page.body}</ArticleBody>
    </div>
  )
}
