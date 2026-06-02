import Link from 'next/link'
import type { Tag } from '@/lib/types'

export function TagBadge({ tag }: { tag: Tag }) {
  return (
    <Link
      href={`/tags/${tag.slug}`}
      className="inline-block px-2 py-0.5 text-xs font-medium bg-gray-100 text-gray-700 rounded hover:bg-gray-200 transition-colors"
    >
      {tag.name}
    </Link>
  )
}
