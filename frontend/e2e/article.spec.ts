// frontend/e2e/article.spec.ts
import { test, expect } from '@playwright/test'

test.describe('Article page', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/articles')
  })

  test('has correct title', async ({ page }) => {
    await expect(page).toHaveTitle(/News/)
  })

  test('displays Articles heading', async ({ page }) => {
    await expect(page.locator('h1', { hasText: 'Articles' })).toBeVisible()
  })

  test('has header with site name', async ({ page }) => {
    const header = page.locator('header').first()
    await expect(header).toBeVisible()
  })

  test('has footer', async ({ page }) => {
    const footer = page.locator('footer').first()
    await expect(footer).toBeVisible()
  })

  test('shows no articles message or article list', async ({ page }) => {
    const noArticles = page.locator('text=No articles found.')
    const articleList = page.locator('article')
    const eitherVisible = await noArticles.isVisible().catch(() => false) ||
      await articleList.count().then(c => c > 0)
    expect(eitherVisible).toBe(true)
  })

  test('navigates back to homepage from header', async ({ page }) => {
    const siteLink = page.locator('header a').first()
    await siteLink.click()
    await expect(page).toHaveURL('/')
  })
})
