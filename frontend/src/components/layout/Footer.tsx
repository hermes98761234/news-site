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
