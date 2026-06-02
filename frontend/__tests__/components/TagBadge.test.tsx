import { it, expect } from 'vitest'
import { render, screen } from '@testing-library/react'
import { TagBadge } from '../../src/components/common/TagBadge'

it('renders tag name as link', () => {
  render(<TagBadge tag={{ id: 1, slug: 'rust', name: 'Rust' }} />)
  const link = screen.getByRole('link', { name: 'Rust' })
  expect(link).toHaveAttribute('href', '/tags/rust')
})
