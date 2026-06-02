// frontend/src/components/layout/Header.tsx
import Link from 'next/link'
import { api } from '@/lib/api'

export async function Header() {
  const settings = await api.settings.get()
  const siteName = settings.site_name ?? 'News'

  return (
    <header className="border-b border-gray-200 bg-white sticky top-0 z-10">
      <div className="max-w-4xl mx-auto px-4 py-4 flex items-center justify-between">
        <Link href="/" className="text-2xl font-serif font-bold text-gray-900">
          {siteName}
        </Link>
        <nav className="flex items-center gap-6 text-sm">
          <Link href="/articles" className="text-gray-600 hover:text-gray-900">Articles</Link>
        </nav>
      </div>
    </header>
  )
}
