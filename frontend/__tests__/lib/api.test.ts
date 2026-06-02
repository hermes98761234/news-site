// frontend/__tests__/lib/api.test.ts
import { describe, it, expect, beforeAll, afterAll, afterEach } from 'vitest'
import { setupServer } from 'msw/node'
import { http, HttpResponse } from 'msw'
import { api } from '../../src/lib/api'

const server = setupServer(
  http.get('http://localhost:3000/api/articles', () =>
    HttpResponse.json({ items: [], total: 0, page: 1, limit: 20 })
  ),
  http.get('http://localhost:3000/api/articles/hello-world', () =>
    HttpResponse.json({
      article: { id: 1, slug: 'hello-world', title: 'Hello', excerpt: '', body: '# Hello',
        author_name: 'Alice', status: 'published', category_id: null,
        published_at: '2026-01-01', created_at: '2026-01-01', updated_at: '2026-01-01' },
      tags: [],
      category: null,
    })
  )
)

beforeAll(() => server.listen())
afterEach(() => server.resetHandlers())
afterAll(() => server.close())

describe('api.articles.list', () => {
  it('returns paginated articles', async () => {
    const result = await api.articles.list()
    expect(result.total).toBe(0)
    expect(result.items).toEqual([])
  })
})

describe('api.articles.get', () => {
  it('returns article with tags', async () => {
    const result = await api.articles.get('hello-world')
    expect(result.article.title).toBe('Hello')
    expect(result.tags).toEqual([])
  })
})
