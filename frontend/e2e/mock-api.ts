// frontend/e2e/mock-api.ts
import http from 'http'

const mockArticles = [
  {
    id: 1,
    slug: 'hello-world',
    title: 'Hello World',
    excerpt: 'A sample article for testing',
    body: '# Hello World\n\nThis is a test article.',
    author_name: 'Test Author',
    status: 'published',
    category_id: 1,
    published_at: '2026-01-15T00:00:00Z',
    created_at: '2026-01-14T00:00:00Z',
    updated_at: '2026-01-14T00:00:00Z',
  },
  {
    id: 2,
    slug: 'second-post',
    title: 'Second Post',
    excerpt: 'Another test article',
    body: '## Second Post\n\nMore content here.',
    author_name: 'Jane Doe',
    status: 'published',
    category_id: 1,
    published_at: '2026-02-01T00:00:00Z',
    created_at: '2026-01-31T00:00:00Z',
    updated_at: '2026-01-31T00:00:00Z',
  },
]

const mockTags = [
  { id: 1, slug: 'news', name: 'News' },
  { id: 2, slug: 'tech', name: 'Tech' },
]

const mockCategories = [
  { id: 1, slug: 'general', name: 'General', description: 'General news' },
]

const mockSettings = [
  { key: 'site_name', value: 'News' },
  { key: 'site_description', value: 'Latest news and articles' },
]

const mockPages = [
  {
    id: 1,
    slug: 'about',
    title: 'About Us',
    body: 'This is the about page.',
    status: 'published',
    created_at: '2026-01-01T00:00:00Z',
    updated_at: '2026-01-01T00:00:00Z',
  },
]

const server = http.createServer((req, res) => {
  res.setHeader('Content-Type', 'application/json')
  res.setHeader('Access-Control-Allow-Origin', '*')

  const url = req.url ?? ''

  // Single article — must come BEFORE /api/articles list
  if (url.match(/^\/api\/articles\/[\w-]+$/)) {
    const slug = url.split('/').pop()!
    const article = mockArticles.find((a) => a.slug === slug)
    if (article) {
      res.end(
        JSON.stringify({
          article,
          tags: [mockTags[0]],
          category: mockCategories[0],
        })
      )
    } else {
      res.statusCode = 404
      res.end(JSON.stringify({ error: 'not found' }))
    }
    return
  }

  if (url.startsWith('/api/articles')) {
    const parsed = new URL(url, 'http://localhost')
    const limit = parseInt(parsed.searchParams.get('limit') ?? '20')
    const page = parseInt(parsed.searchParams.get('page') ?? '1')
    const items = mockArticles.map((a) => ({
      article: a,
      tags: [mockTags[0]],
      category: mockCategories[0],
    }))
    res.end(
      JSON.stringify({
        items: items.slice(0, limit),
        total: items.length,
        page,
        limit,
      })
    )
    return
  }

  if (url === '/api/tags') {
    res.end(JSON.stringify(mockTags))
    return
  }

  if (url.match(/^\/api\/tags\/[\w-]+\/articles$/)) {
    const slug = url.split('/')[3]
    const tag = mockTags.find((t) => t.slug === slug)
    if (tag) {
      res.end(
        JSON.stringify(
          mockArticles.map((a) => ({
            article: a,
            tags: [tag],
            category: mockCategories[0],
          }))
        )
      )
    } else {
      res.end(JSON.stringify([]))
    }
    return
  }

  if (url === '/api/categories') {
    res.end(JSON.stringify(mockCategories))
    return
  }

  if (url.match(/^\/api\/pages\/[\w-]+$/)) {
    const slug = url.split('/').pop()!
    const page = mockPages.find((p) => p.slug === slug)
    if (page) {
      res.end(JSON.stringify(page))
    } else {
      res.statusCode = 404
      res.end(JSON.stringify({ error: 'not found' }))
    }
    return
  }

  if (url === '/api/settings') {
    res.end(JSON.stringify(mockSettings))
    return
  }

  res.statusCode = 404
  res.end(JSON.stringify({ error: 'not found' }))
})

const PORT = 3001
server.listen(PORT, () => {
  console.log(`Mock API server running on http://localhost:${PORT}`)
})
