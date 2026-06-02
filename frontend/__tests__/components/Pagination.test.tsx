import { it, expect } from 'vitest'
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
