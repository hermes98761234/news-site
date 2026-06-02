import { it, expect, describe } from 'vitest'
import { render, screen } from '@testing-library/react'
import { ArticleList } from '../../src/components/common/ArticleList'
import type { PaginatedArticles } from '../../src/lib/types'

const mockPaginated = (
  overrides: Partial<PaginatedArticles> = {},
): PaginatedArticles => ({
  items: [
    {
      id: 1,
      slug: 'first-post',
      title: 'First Post',
      excerpt: 'First excerpt',
      body: '# First',
      author_name: 'Author',
      status: 'published',
      category_id: null,
      published_at: '2024-01-01T00:00:00Z',
      created_at: '2024-01-01T00:00:00Z',
      updated_at: '2024-01-01T00:00:00Z',
      tags: [],
      category: null,
    },
    {
      id: 2,
      slug: 'second-post',
      title: 'Second Post',
      excerpt: 'Second excerpt',
      body: '# Second',
      author_name: 'Author',
      status: 'published',
      category_id: null,
      published_at: null,
      created_at: '2024-02-01T00:00:00Z',
      updated_at: '2024-02-01T00:00:00Z',
      tags: [],
      category: null,
    },
  ],
  total: 2,
  page: 1,
  limit: 20,
  ...overrides,
})

describe('ArticleList', () => {
  it('renders article cards for each item', () => {
    render(<ArticleList paginated={mockPaginated()} />)
    expect(screen.getByText('First Post')).toBeInTheDocument()
    expect(screen.getByText('Second Post')).toBeInTheDocument()
  })

  it('renders empty message when no items', () => {
    render(<ArticleList paginated={mockPaginated({ items: [], total: 0 })} />)
    expect(screen.getByText('No articles found.')).toBeInTheDocument()
  })

  it('renders pagination', () => {
    render(<ArticleList paginated={mockPaginated({ total: 40 })} />)
    expect(screen.getByText(/next/i)).toBeInTheDocument()
  })

  it('passes basePath to Pagination', () => {
    render(<ArticleList paginated={mockPaginated({ total: 40 })} basePath="/tags/rust" />)
    const nextLink = screen.getByRole('link', { name: /next/i })
    expect(nextLink).toHaveAttribute('href', '/tags/rust?page=2')
  })
})
