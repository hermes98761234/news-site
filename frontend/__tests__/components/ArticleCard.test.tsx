import { it, expect, describe } from 'vitest'
import { render, screen } from '@testing-library/react'
import { ArticleCard } from '../../src/components/common/ArticleCard'

const mockArticle = (
  overrides: Partial<import('../../src/lib/types').ArticleWithTags> = {},
): import('../../src/lib/types').ArticleWithTags => ({
  id: 1,
  slug: 'hello-world',
  title: 'Hello World',
  excerpt: 'A test article excerpt',
  body: '# Hello',
  author_name: 'Jane',
  status: 'published',
  category_id: 1,
  published_at: '2024-03-15T00:00:00Z',
  created_at: '2024-03-14T00:00:00Z',
  updated_at: '2024-03-14T00:00:00Z',
  tags: [
    { id: 1, slug: 'rust', name: 'Rust' },
    { id: 2, slug: 'webdev', name: 'WebDev' },
  ],
  category: {
    id: 1,
    slug: 'tech',
    name: 'Tech',
    description: '',
  },
  ...overrides,
})

describe('ArticleCard', () => {
  it('renders article title as link', () => {
    render(<ArticleCard article={mockArticle()} />)
    const link = screen.getByRole('link', { name: 'Hello World' })
    expect(link).toHaveAttribute('href', '/articles/hello-world')
  })

  it('renders excerpt', () => {
    render(<ArticleCard article={mockArticle()} />)
    expect(screen.getByText('A test article excerpt')).toBeInTheDocument()
  })

  it('renders published date', () => {
    render(<ArticleCard article={mockArticle()} />)
    expect(screen.getByText(/Mar 15, 2024/)).toBeInTheDocument()
  })

  it('renders author name', () => {
    render(<ArticleCard article={mockArticle()} />)
    expect(screen.getByText('by Jane')).toBeInTheDocument()
  })

  it('renders tag badges', () => {
    render(<ArticleCard article={mockArticle()} />)
    expect(screen.getByRole('link', { name: 'Rust' })).toHaveAttribute('href', '/tags/rust')
    expect(screen.getByRole('link', { name: 'WebDev' })).toHaveAttribute('href', '/tags/webdev')
  })

  it('renders category badge', () => {
    render(<ArticleCard article={mockArticle()} />)
    expect(screen.getByRole('link', { name: 'Tech' })).toHaveAttribute('href', '/categories/tech')
  })

  it('falls back to created_at when published_at is null', () => {
    const article = mockArticle({ published_at: null })
    render(<ArticleCard article={article} />)
    expect(screen.getByText(/Mar 14, 2024/)).toBeInTheDocument()
  })

  it('renders without excerpt when empty', () => {
    const article = mockArticle({ excerpt: '' })
    render(<ArticleCard article={article} />)
    expect(screen.queryByText('A test article excerpt')).toBeNull()
  })

  it('renders with empty tags', () => {
    const article = mockArticle({ tags: [] })
    render(<ArticleCard article={article} />)
    expect(screen.queryAllByRole('link').length).toBe(2) // title link + category
  })

  it('renders when category is null', () => {
    const article = mockArticle({ category: null })
    render(<ArticleCard article={article} />)
    expect(screen.getByRole('link', { name: 'Hello World' })).toBeInTheDocument()
  })
})
