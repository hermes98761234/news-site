'use client'

import ReactMarkdown from 'react-markdown'
import rehypeHighlight from 'rehype-highlight'
import remarkGfm from 'remark-gfm'

interface ArticleBodyProps {
  children: string
}

export function ArticleBody({ children }: ArticleBodyProps) {
  return (
    <div className="prose prose-lg max-w-none prose-headings:font-serif prose-a:text-blue-600 prose-code:text-pink-600">
      <ReactMarkdown
        remarkPlugins={[remarkGfm]}
        rehypePlugins={[rehypeHighlight]}
      >
        {children}
      </ReactMarkdown>
    </div>
  )
}
