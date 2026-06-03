// frontend/src/app/[slug]/page.tsx
import type { Metadata } from 'next'
import { api } from '@/lib/api'
import { ArticleBody } from '@/components/common/ArticleBody'
import type { Page } from '@/lib/types'

interface Props { params: { slug: string } }

export const dynamicParams = false

export async function generateStaticParams() {
  try {
    const pages = await api.pages.list()
    return pages.map(p => ({ slug: p.slug }))
  } catch {
    return []
  }
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
  let page: Page | undefined
  try {
    page = await api.pages.get(params.slug)
  } catch {
    // API unavailable or page not found
  }
  if (!page) {
    return <div className="max-w-2xl"><p className="text-gray-500">Page not found.</p></div>
  }
  return (
    <div className="max-w-2xl">
      <h1 className="text-4xl font-serif font-bold mb-6">{page.title}</h1>
      <ArticleBody>{page.body}</ArticleBody>
    </div>
  )
}
