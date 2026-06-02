// frontend/e2e/homepage.spec.ts
import { test, expect } from '@playwright/test'

test.describe('Homepage', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/')
  })

  test('has correct title', async ({ page }) => {
    await expect(page).toHaveTitle(/News/)
  })

  test('displays site name in header', async ({ page }) => {
    const header = page.locator('header').first()
    await expect(header).toBeVisible()
  })

  test('has a link to Articles in the navigation', async ({ page }) => {
    const navLink = page.locator('header nav a', { hasText: 'Articles' })
    await expect(navLink).toBeVisible()
    await expect(navLink).toHaveAttribute('href', '/articles/')
  })

  test('shows Latest Articles heading', async ({ page }) => {
    await expect(page.locator('h1', { hasText: 'Latest Articles' })).toBeVisible()
  })

  test('has a footer', async ({ page }) => {
    const footer = page.locator('footer').first()
    await expect(footer).toBeVisible()
  })

  test('navigates to Articles page when clicking nav link', async ({ page }) => {
    await page.locator('header nav a', { hasText: 'Articles' }).click()
    await expect(page).toHaveURL('/articles/')
    await expect(page.locator('h1', { hasText: 'Articles' })).toBeVisible()
  })
})
