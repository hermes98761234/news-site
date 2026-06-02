# News Site — Phase 3: Frontend Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build the Next.js frontend — modern minimal news site with homepage, article listing, single article, tags, categories, static pages. Fully tested with React Testing Library + Playwright.

**Architecture:** Next.js 14 App Router with `output: 'export'` for static generation. Fetches from Rust API at build time via `generateStaticParams`. shadcn/ui + Tailwind for styling. react-markdown for article body rendering.

**Tech Stack:** Next.js 14, TypeScript, Tailwind CSS, shadcn/ui, react-markdown, rehype-highlight, Vitest + React Testing Library, Playwright, msw

**Prerequisite:** Phase 1 backend must be running for `next build` to succeed (fetches slugs at build time).

---

## File Map

```
frontend/
├── next.config.ts
├── tailwind.config.ts
├── tsconfig.json
├── package.json
├── src/
│   ├── app/
│   │   ├── layout.tsx              # root layout: Header + Footer
│   │   ├── page.tsx                # homepage
│   │   ├── articles/
│   │   │   ├── page.tsx            # article listing
│   │   │   └── [slug]/
│   │   │       └── page.tsx        # single article
│   │   ├── categories/
│   │   │   └── [slug]/
│   │   │       └── page.tsx        # articles by category
│   │   ├── tags/
│   │   │   └── [slug]/
│   │   │       └── page.tsx        # articles by tag
│   │   └── [slug]/
│   │       └── page.tsx            # static pages (about, contact)
│   ├── components/
│   │   ├── layout/
│   │   │   ├── Header.tsx
│   │   │   ├── Footer.tsx
│   │   │   └── Nav.tsx
│   │   ├── article/
│   │   │   ├── ArticleCard.tsx
│   │   │   ├── ArticleList.tsx
│   │   │   └── ArticleBody.tsx
│   │   └── common/
│   │       ├── Pagination.tsx
│   │       ├── TagBadge.tsx
│   │       └── CategoryBadge.tsx
│   ├── lib/
│   │   ├── api.ts                  # fetch wrappers for all API endpoints
│   │   └── types.ts                # TypeScript types matching Rust models
│   └── styles/
│       └── globals.css
├── __tests__/
│   ├── components/
│   │   ├── ArticleCard.test.tsx
│   │   ├── Pagination.test.tsx
│   │   └── TagBadge.test.tsx
│   └── lib/
│       └── api.test.ts
└── e2e/
    ├── homepage.spec.ts
    ├── article.spec.ts
    └── tag-filter.spec.ts
```

---

### Task 1: Project scaffold

**Files:**
- Create: `frontend/package.json`
- Create: `frontend/next.config.ts`
- Create: `frontend/tailwind.config.ts`
- Create: `frontend/tsconfig.json`

- [ ] **Step 1: Create package.json**

```json
{
  "name": "news-site-frontend",
  "version": "0.1.0",
  "private": true,
  "scripts": {
    "dev": "next dev",
    "build": "next build",
    "start": "next start",
    "test": "vitest run",
    "test:watch": "vitest",
    "test:e2e": "playwright test",
    "lint": "next lint"
  },
  "dependencies": {
    "next": "14.2.0",
    "react": "^18",
    "react-dom": "^18",
    "react-markdown": "^9",
    "rehype-highlight": "^7",
    "remark-gfm": "^4"
  },
  "devDependencies": {
    "@testing-library/jest-dom": "^6",
    "@testing-library/react": "^15",
    "@testing-library/user-event": "^14",
    "@playwright/test": "^1.44",
    "@types/node": "^20",
    "@types/react": "^18",
    "@types/react-dom": "^18",
    "autoprefixer": "^10",
    "msw": "^2",
    "postcss": "^8",
    "tailwindcss": "^3",
    "typescript": "^5",
    "vitest": "^1",
    "@vitejs/plugin-react": "^4",
    "jsdom": "^24"
  }
}
```

- [ ] **Step 2: Create next.config.ts**

```typescript
// frontend/next.config.ts
import type { NextConfig } from 'next'

const nextConfig: NextConfig = {
  output: 'export',
  trailingSlash: true,
  images: { unoptimized: true },
  env: {
    NEXT_PUBLIC_API_URL: process.env.NEXT_PUBLIC_API_URL ?? 'http://localhost:3000',
  },
}

export default nextConfig
```

- [ ] **Step 3: Create tailwind.config.ts**

```typescript
// frontend/tailwind.config.ts
import type { Config } from 'tailwindcss'

export default {
  content: ['./src/**/*.{ts,tsx}'],
  theme: {
    extend: {
      fontFamily: {
        serif: ['Georgia', 'serif'],
        sans: ['Inter', 'system-ui', 'sans-serif'],
      },
    },
  },
  plugins: [],
} satisfies Config
```

- [ ] **Step 4: Install dependencies**

```bash
cd frontend && npm install
```
Expected: `node_modules` created, no errors

- [ ] **Step 5: Commit**

```bash
git add frontend/package.json frontend/next.config.ts frontend/tailwind.config.ts frontend/tsconfig.json
git commit -m "feat: scaffold next.js frontend"
```

---

### Task 2: Types and API client

**Files:**
- Create: `frontend/src/lib/types.ts`
- Create: `frontend/src/lib/api.ts`

- [ ] **Step 1: Write types.ts**

```typescript
// frontend/src/lib/types.ts
export interface Tag {
  id: number
  slug: string
  name: string
}

export interface Category {
  id: number
  slug: string
  name: string
  description: string
}

export interface Article {
  id: number
  slug: string
  title: string
  excerpt: string
  body: string
  author_name: string
  status: 'draft' | 'published' | 'archived'
  category_id: number | null
  published_at: string | null
  created_at: string
  updated_at: string
}

export interface ArticleWithTags {
  article: Article
  tags: Tag[]
  category: Category | null
}

export interface PaginatedArticles {
  items: ArticleWithTags[]
  total: number
  page: number
  limit: number
}

export interface Page {
  id: number
  slug: string
  title: string
  body: string
  status: 'draft' | 'published'
  created_at: string
  updated_at: string
}

export interface Setting {
  key: string
  value: string
}
```

- [ ] **Step 2: Write api.ts**

```typescript
// frontend/src/lib/api.ts
import type { ArticleWithTags, Category, Page, PaginatedArticles, Setting, Tag } from './types'

const API_URL = process.env.NEXT_PUBLIC_API_URL ?? 'http://localhost:3000'

async function fetchApi<T>(path: string): Promise<T> {
  const res = await fetch(`${API_URL}/api${path}`, { next: { revalidate: 300 } })
  if (!res.ok) throw new Error(`API error ${res.status}: ${path}`)
  return res.json()
}

export const api = {
  articles: {
    list: (page = 1, limit = 20, tag?: string, category?: string) => {
      const params = new URLSearchParams({ page: String(page), limit: String(limit) })
      if (tag) params.set('tag', tag)
      if (category) params.set('category', category)
      return fetchApi<PaginatedArticles>(`/articles?${params}`)
    },
    get: (slug: string) => fetchApi<ArticleWithTags>(`/articles/${slug}`),
    slugs: async () => {
      const data = await fetchApi<PaginatedArticles>('/articles?limit=1000')
      return data.items.map(i => i.article.slug)
    },
  },
  pages: {
    get: (slug: string) => fetchApi<Page>(`/pages/${slug}`),
  },
  tags: {
    list: () => fetchApi<Tag[]>('/tags'),
    articles: (slug: string) => fetchApi<ArticleWithTags[]>(`/tags/${slug}/articles`),
  },
  categories: {
    list: () => fetchApi<Category[]>('/categories'),
    articles: (slug: string, category: string) =>
      fetchApi<PaginatedArticles>(`/articles?category=${category}`),
  },
  settings: {
    get: async () => {
      const settings = await fetchApi<Setting[]>('/settings')
      return Object.fromEntries(settings.map(s => [s.key, s.value])) as Record<string, string>
    },
  },
}
```

- [ ] **Step 3: Write failing test for api.ts**

```typescript
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
```

- [ ] **Step 4: Run test**

```bash
cd frontend && npm test
```
Expected: api.test.ts passes

- [ ] **Step 5: Commit**

```bash
git add frontend/src/lib/ frontend/__tests__/lib/
git commit -m "feat: add frontend types and api client"
```

---

### Task 3: Common components

**Files:**
- Create: `frontend/src/components/common/TagBadge.tsx`
- Create: `frontend/src/components/common/CategoryBadge.tsx`
- Create: `frontend/src/components/common/Pagination.tsx`

- [ ] **Step 1: Write failing test for TagBadge**

```typescript
// frontend/__tests__/components/TagBadge.test.tsx
import { render, screen } from '@testing-library/react'
import { TagBadge } from '../../src/components/common/TagBadge'

it('renders tag name as link', () => {
  render(<TagBadge tag={{ id: 1, slug: 'rust', name: 'Rust' }} />)
  const link = screen.getByRole('link', { name: 'Rust' })
  expect(link).toHaveAttribute('href', '/tags/rust')
})
```

- [ ] **Step 2: Write TagBadge.tsx**

```tsx
// frontend/src/components/common/TagBadge.tsx
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
```

- [ ] **Step 3: Run test**

```bash
cd frontend && npm test -- TagBadge
```
Expected: passes

- [ ] **Step 4: Write failing test for Pagination**

```typescript
// frontend/__tests__/components/Pagination.test.tsx
import { render, screen } from '@testing-library/react'
import { Pagination } from '../../src/components/common/Pagination'

it('renders next link when more pages exist', () => {
  render(<Pagination page={1} total={50} limit={20} basePath="/articles" />)
  expect(screen.getByRole('link', { name: /next/i })).toHaveAttribute('href', '/articles?page=2')
})

it('renders prev link when not on first page', () => {
  render(<Pagination page={2} total={50} limit={20} basePath="/articles" />)
  expect(screen.getByRole('link', { name: /prev/i })).toHaveAttribute('href', '/articles?page=1')
})

it('hides next when on last page', () => {
  render(<Pagination page={3} total={50} limit={20} basePath="/articles" />)
  expect(screen.queryByRole('link', { name: /next/i })).toBeNull()
})
```

- [ ] **Step 5: Write Pagination.tsx**

```tsx
// frontend/src/components/common/Pagination.tsx
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
```

- [ ] **Step 6: Write CategoryBadge.tsx**

```tsx
// frontend/src/components/common/CategoryBadge.tsx
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
```

- [ ] **Step 7: Run all tests**

```bash
cd frontend && npm test
```
Expected: all pass

- [ ] **Step 8: Commit**

```bash
git add frontend/src/components/common/ frontend/__tests__/components/
git commit -m "feat: add common UI components"
```

---

### Task 4: Article components

**Files:**
- Create: `frontend/src/components/article/ArticleCard.tsx`
- Create: `frontend/src/components/article/ArticleList.tsx`
- Create: `frontend/src/components/article/ArticleBody.tsx`

- [ ] **Step 1: Write failing test for ArticleCard**

```typescript
// frontend/__tests__/components/ArticleCard.test.tsx
import { render, screen } from '@testing-library/react'
import { ArticleCard } from '../../src/components/article/ArticleCard'
import type { ArticleWithTags } from '../../src/lib/types'

const mockArticle: ArticleWithTags = {
  article: {
    id: 1, slug: 'test-article', title: 'Test Article', excerpt: 'A summary',
    body: '', author_name: 'Alice', status: 'published',
    category_id: null, published_at: '2026-01-15T00:00:00', created_at: '2026-01-15T00:00:00', updated_at: '2026-01-15T00:00:00',
  },
  tags: [{ id: 1, slug: 'rust', name: 'Rust' }],
  category: null,
}

it('renders article title as link', () => {
  render(<ArticleCard item={mockArticle} />)
  const link = screen.getByRole('link', { name: 'Test Article' })
  expect(link).toHaveAttribute('href', '/articles/test-article')
})

it('renders excerpt', () => {
  render(<ArticleCard item={mockArticle} />)
  expect(screen.getByText('A summary')).toBeInTheDocument()
})

it('renders author name', () => {
  render(<ArticleCard item={mockArticle} />)
  expect(screen.getByText('Alice')).toBeInTheDocument()
})

it('renders tags', () => {
  render(<ArticleCard item={mockArticle} />)
  expect(screen.getByRole('link', { name: 'Rust' })).toBeInTheDocument()
})
```

- [ ] **Step 2: Write ArticleCard.tsx**

```tsx
// frontend/src/components/article/ArticleCard.tsx
import Link from 'next/link'
import type { ArticleWithTags } from '@/lib/types'
import { TagBadge } from '@/components/common/TagBadge'
import { CategoryBadge } from '@/components/common/CategoryBadge'

export function ArticleCard({ item }: { item: ArticleWithTags }) {
  const { article, tags, category } = item
  const date = article.published_at
    ? new Date(article.published_at).toLocaleDateString('en-US', { year: 'numeric', month: 'long', day: 'numeric' })
    : ''

  return (
    <article className="border-b border-gray-200 py-6 last:border-0">
      <div className="flex items-center gap-2 mb-2">
        {category && <CategoryBadge category={category} />}
        {tags.map(tag => <TagBadge key={tag.id} tag={tag} />)}
      </div>
      <Link href={`/articles/${article.slug}`}>
        <h2 className="text-xl font-serif font-semibold text-gray-900 hover:text-blue-700 mb-1">
          {article.title}
        </h2>
      </Link>
      {article.excerpt && (
        <p className="text-gray-600 text-sm mb-2 leading-relaxed">{article.excerpt}</p>
      )}
      <div className="text-xs text-gray-400">
        <span>{article.author_name}</span>
        {date && <> · <time dateTime={article.published_at ?? ''}>{date}</time></>}
      </div>
    </article>
  )
}
```

- [ ] **Step 3: Write ArticleList.tsx**

```tsx
// frontend/src/components/article/ArticleList.tsx
import type { ArticleWithTags } from '@/lib/types'
import { ArticleCard } from './ArticleCard'

export function ArticleList({ items }: { items: ArticleWithTags[] }) {
  if (items.length === 0) {
    return <p className="text-gray-500 py-8 text-center">No articles found.</p>
  }
  return (
    <div>
      {items.map(item => (
        <ArticleCard key={item.article.id} item={item} />
      ))}
    </div>
  )
}
```

- [ ] **Step 4: Write ArticleBody.tsx**

```tsx
// frontend/src/components/article/ArticleBody.tsx
'use client'
import ReactMarkdown from 'react-markdown'
import remarkGfm from 'remark-gfm'
import rehypeHighlight from 'rehype-highlight'
import 'highlight.js/styles/github.css'

export function ArticleBody({ body }: { body: string }) {
  return (
    <div className="prose prose-gray max-w-none">
      <ReactMarkdown remarkPlugins={[remarkGfm]} rehypePlugins={[rehypeHighlight]}>
        {body}
      </ReactMarkdown>
    </div>
  )
}
```

- [ ] **Step 5: Run tests**

```bash
cd frontend && npm test -- ArticleCard
```
Expected: all 4 tests pass

- [ ] **Step 6: Commit**

```bash
git add frontend/src/components/article/ frontend/__tests__/components/ArticleCard.test.tsx
git commit -m "feat: add article components"
```

---

### Task 5: Layout components

**Files:**
- Create: `frontend/src/components/layout/Header.tsx`
- Create: `frontend/src/components/layout/Footer.tsx`
- Create: `frontend/src/app/layout.tsx`
- Create: `frontend/src/styles/globals.css`

- [ ] **Step 1: Write Header.tsx**

```tsx
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
```

- [ ] **Step 2: Write Footer.tsx**

```tsx
// frontend/src/components/layout/Footer.tsx
import { api } from '@/lib/api'

export async function Footer() {
  const settings = await api.settings.get()
  return (
    <footer className="border-t border-gray-200 mt-16 py-8">
      <div className="max-w-4xl mx-auto px-4 text-center text-sm text-gray-400">
        {settings.site_description && <p className="mb-1">{settings.site_description}</p>}
        <p>© {new Date().getFullYear()} {settings.site_name}</p>
      </div>
    </footer>
  )
}
```

- [ ] **Step 3: Write globals.css**

```css
/* frontend/src/styles/globals.css */
@tailwind base;
@tailwind components;
@tailwind utilities;

@import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600&display=swap');

body {
  font-family: 'Inter', system-ui, sans-serif;
  background-color: #fff;
  color: #111827;
}
```

- [ ] **Step 4: Write app/layout.tsx**

```tsx
// frontend/src/app/layout.tsx
import type { Metadata } from 'next'
import { Header } from '@/components/layout/Header'
import { Footer } from '@/components/layout/Footer'
import '@/styles/globals.css'

export const metadata: Metadata = {
  title: { default: 'News', template: '%s | News' },
  description: 'Latest news',
}

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <body className="min-h-screen flex flex-col bg-white">
        <Header />
        <main className="flex-1 max-w-4xl mx-auto w-full px-4 py-8">
          {children}
        </main>
        <Footer />
      </body>
    </html>
  )
}
```

- [ ] **Step 5: Commit**

```bash
git add frontend/src/components/layout/ frontend/src/app/layout.tsx frontend/src/styles/
git commit -m "feat: add layout components"
```

---

### Task 6: Pages — homepage, article listing, single article

**Files:**
- Create: `frontend/src/app/page.tsx`
- Create: `frontend/src/app/articles/page.tsx`
- Create: `frontend/src/app/articles/[slug]/page.tsx`

- [ ] **Step 1: Write homepage**

```tsx
// frontend/src/app/page.tsx
import { api } from '@/lib/api'
import { ArticleList } from '@/components/article/ArticleList'
import { TagBadge } from '@/components/common/TagBadge'

export default async function HomePage() {
  const [articles, tags] = await Promise.all([
    api.articles.list(1, 10),
    api.tags.list(),
  ])

  return (
    <div className="grid grid-cols-1 md:grid-cols-3 gap-8">
      <section className="md:col-span-2">
        <h1 className="text-3xl font-serif font-bold mb-6">Latest</h1>
        <ArticleList items={articles.items} />
      </section>
      <aside>
        <h2 className="text-sm font-semibold uppercase tracking-wider text-gray-500 mb-3">Topics</h2>
        <div className="flex flex-wrap gap-2">
          {tags.map(tag => <TagBadge key={tag.id} tag={tag} />)}
        </div>
      </aside>
    </div>
  )
}
```

- [ ] **Step 2: Write article listing page**

```tsx
// frontend/src/app/articles/page.tsx
import { api } from '@/lib/api'
import { ArticleList } from '@/components/article/ArticleList'
import { Pagination } from '@/components/common/Pagination'

interface Props {
  searchParams: { page?: string; tag?: string; category?: string }
}

export default async function ArticlesPage({ searchParams }: Props) {
  const page = Number(searchParams.page ?? 1)
  const { items, total, limit } = await api.articles.list(
    page, 20, searchParams.tag, searchParams.category
  )

  return (
    <div>
      <h1 className="text-3xl font-serif font-bold mb-6">Articles</h1>
      <ArticleList items={items} />
      <Pagination page={page} total={total} limit={limit} basePath="/articles" />
    </div>
  )
}
```

- [ ] **Step 3: Write single article page**

```tsx
// frontend/src/app/articles/[slug]/page.tsx
import type { Metadata } from 'next'
import { notFound } from 'next/navigation'
import { api } from '@/lib/api'
import { ArticleBody } from '@/components/article/ArticleBody'
import { TagBadge } from '@/components/common/TagBadge'
import { CategoryBadge } from '@/components/common/CategoryBadge'

interface Props { params: { slug: string } }

export async function generateStaticParams() {
  const slugs = await api.articles.slugs()
  return slugs.map(slug => ({ slug }))
}

export async function generateMetadata({ params }: Props): Promise<Metadata> {
  try {
    const { article } = await api.articles.get(params.slug)
    return { title: article.title, description: article.excerpt }
  } catch {
    return {}
  }
}

export default async function ArticlePage({ params }: Props) {
  let data
  try {
    data = await api.articles.get(params.slug)
  } catch {
    notFound()
  }
  const { article, tags, category } = data
  const date = article.published_at
    ? new Date(article.published_at).toLocaleDateString('en-US', { year: 'numeric', month: 'long', day: 'numeric' })
    : ''

  return (
    <article className="max-w-2xl">
      <div className="flex items-center gap-2 mb-3">
        {category && <CategoryBadge category={category} />}
        {tags.map(tag => <TagBadge key={tag.id} tag={tag} />)}
      </div>
      <h1 className="text-4xl font-serif font-bold text-gray-900 mb-3 leading-tight">
        {article.title}
      </h1>
      <div className="text-sm text-gray-500 mb-6">
        <span>{article.author_name}</span>
        {date && <> · <time>{date}</time></>}
      </div>
      {article.excerpt && (
        <p className="text-lg text-gray-600 mb-6 leading-relaxed border-l-4 border-gray-200 pl-4">
          {article.excerpt}
        </p>
      )}
      <ArticleBody body={article.body} />
    </article>
  )
}
```

- [ ] **Step 4: Commit**

```bash
git add frontend/src/app/
git commit -m "feat: add homepage, article listing, single article pages"
```

---

### Task 7: Tags, categories, static pages

**Files:**
- Create: `frontend/src/app/tags/[slug]/page.tsx`
- Create: `frontend/src/app/categories/[slug]/page.tsx`
- Create: `frontend/src/app/[slug]/page.tsx`

- [ ] **Step 1: Write tags/[slug]/page.tsx**

```tsx
// frontend/src/app/tags/[slug]/page.tsx
import type { Metadata } from 'next'
import { notFound } from 'next/navigation'
import { api } from '@/lib/api'
import { ArticleList } from '@/components/article/ArticleList'

interface Props { params: { slug: string } }

export async function generateStaticParams() {
  const tags = await api.tags.list()
  return tags.map(t => ({ slug: t.slug }))
}

export async function generateMetadata({ params }: Props): Promise<Metadata> {
  return { title: `Tag: ${params.slug}` }
}

export default async function TagPage({ params }: Props) {
  let items
  try {
    items = await api.tags.articles(params.slug)
  } catch {
    notFound()
  }
  return (
    <div>
      <h1 className="text-3xl font-serif font-bold mb-6">#{params.slug}</h1>
      <ArticleList items={items} />
    </div>
  )
}
```

- [ ] **Step 2: Write categories/[slug]/page.tsx**

```tsx
// frontend/src/app/categories/[slug]/page.tsx
import type { Metadata } from 'next'
import { notFound } from 'next/navigation'
import { api } from '@/lib/api'
import { ArticleList } from '@/components/article/ArticleList'

interface Props { params: { slug: string } }

export async function generateStaticParams() {
  const cats = await api.categories.list()
  return cats.map(c => ({ slug: c.slug }))
}

export async function generateMetadata({ params }: Props): Promise<Metadata> {
  return { title: params.slug }
}

export default async function CategoryPage({ params }: Props) {
  let data
  try {
    data = await api.categories.articles(params.slug, params.slug)
  } catch {
    notFound()
  }
  return (
    <div>
      <h1 className="text-3xl font-serif font-bold mb-6 capitalize">{params.slug}</h1>
      <ArticleList items={data.items} />
    </div>
  )
}
```

- [ ] **Step 3: Write [slug]/page.tsx (static pages)**

```tsx
// frontend/src/app/[slug]/page.tsx
import type { Metadata } from 'next'
import { notFound } from 'next/navigation'
import { api } from '@/lib/api'
import { ArticleBody } from '@/components/article/ArticleBody'

interface Props { params: { slug: string } }

// Static pages don't need generateStaticParams — Next.js will try at request time
// For full static export, add known page slugs here:
export async function generateStaticParams() {
  return []  // populated at build time if pages exist
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
      <ArticleBody body={page.body} />
    </div>
  )
}
```

- [ ] **Step 4: Commit**

```bash
git add frontend/src/app/tags/ frontend/src/app/categories/ frontend/src/app/\[slug\]/
git commit -m "feat: add tag, category, and static page routes"
```

---

### Task 8: Vitest config + E2E tests

**Files:**
- Create: `frontend/vitest.config.ts`
- Create: `frontend/vitest.setup.ts`
- Create: `frontend/playwright.config.ts`
- Create: `frontend/e2e/homepage.spec.ts`
- Create: `frontend/e2e/article.spec.ts`

- [ ] **Step 1: Write vitest.config.ts**

```typescript
// frontend/vitest.config.ts
import { defineConfig } from 'vitest/config'
import react from '@vitejs/plugin-react'
import path from 'path'

export default defineConfig({
  plugins: [react()],
  test: {
    environment: 'jsdom',
    setupFiles: ['./vitest.setup.ts'],
    globals: true,
  },
  resolve: {
    alias: { '@': path.resolve(__dirname, './src') },
  },
})
```

- [ ] **Step 2: Write vitest.setup.ts**

```typescript
// frontend/vitest.setup.ts
import '@testing-library/jest-dom'
```

- [ ] **Step 3: Write playwright.config.ts**

```typescript
// frontend/playwright.config.ts
import { defineConfig } from '@playwright/test'

export default defineConfig({
  testDir: './e2e',
  use: {
    baseURL: 'http://localhost:3000',
  },
  webServer: {
    command: 'npm run dev',
    url: 'http://localhost:3000',
    reuseExistingServer: true,
  },
})
```

- [ ] **Step 4: Write e2e/homepage.spec.ts**

```typescript
// frontend/e2e/homepage.spec.ts
import { test, expect } from '@playwright/test'

test('homepage loads and shows latest heading', async ({ page }) => {
  await page.goto('/')
  await expect(page.getByRole('heading', { name: 'Latest' })).toBeVisible()
})

test('clicking article title navigates to article page', async ({ page }) => {
  await page.goto('/')
  const firstArticle = page.locator('article').first()
  const titleLink = firstArticle.getByRole('link').first()
  const href = await titleLink.getAttribute('href')
  await titleLink.click()
  await expect(page).toHaveURL(href!)
})
```

- [ ] **Step 5: Write e2e/article.spec.ts**

```typescript
// frontend/e2e/article.spec.ts
import { test, expect } from '@playwright/test'

test('article page shows title and body', async ({ page }) => {
  // Navigate to articles list first to find a real slug
  await page.goto('/articles')
  const firstLink = page.locator('article a').first()
  await firstLink.click()
  // Should be on an article page with an h1
  await expect(page.locator('h1')).toBeVisible()
})

test('tag badge links to tag page', async ({ page }) => {
  await page.goto('/articles')
  const tagLink = page.locator('a[href^="/tags/"]').first()
  if (await tagLink.count() > 0) {
    const href = await tagLink.getAttribute('href')
    await tagLink.click()
    await expect(page).toHaveURL(href!)
  }
})
```

- [ ] **Step 6: Run unit tests**

```bash
cd frontend && npm test
```
Expected: all component + api tests pass

- [ ] **Step 7: Commit**

```bash
git add frontend/vitest.config.ts frontend/vitest.setup.ts frontend/playwright.config.ts frontend/e2e/
git commit -m "test: add vitest config and e2e playwright tests"
```

---

### Task 9: Build verification

- [ ] **Step 1: Build Next.js** (backend must be running)

```bash
cd frontend && NEXT_PUBLIC_API_URL=http://localhost:3000 npm run build
```
Expected: `out/` directory created with static HTML files, no build errors

- [ ] **Step 2: Run all unit tests**

```bash
cd frontend && npm test
```
Expected: all pass

- [ ] **Step 3: Commit**

```bash
git commit -m "chore: verify frontend build passes"
```
