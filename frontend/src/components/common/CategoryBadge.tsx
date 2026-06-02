import Link from 'next/link'
import type { Category } from '@/lib/types'

export function CategoryBadge({ category }: { category: Category }) {
  return (
    <Link
      href={`/categories/${category.slug}`}
      className="inline-block px-2 py-0.5 text-xs font-medium bg-blue-50 text-blue-700 rounded hover:bg-blue-100 transition-colors"
    >
      {category.name}
    </Link>
  )
}
