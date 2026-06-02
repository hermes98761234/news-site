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
