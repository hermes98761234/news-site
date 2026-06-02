import Link from 'next/link'

interface Props {
  page: number
  total: number
  limit: number
  basePath: string
}

export function Pagination({ page, total, limit, basePath }: Props) {
  const totalPages = Math.ceil(total / limit)
  return (
    <nav className="flex justify-between items-center mt-8">
      {page > 1 ? (
        <Link href={`${basePath}?page=${page - 1}`} className="text-sm font-medium text-gray-600 hover:text-gray-900">
          ← Prev
        </Link>
      ) : <span />}
      <span className="text-sm text-gray-500">{page} / {totalPages}</span>
      {page < totalPages ? (
        <Link href={`${basePath}?page=${page + 1}`} className="text-sm font-medium text-gray-600 hover:text-gray-900">
          Next →
        </Link>
      ) : <span />}
    </nav>
  )
}
