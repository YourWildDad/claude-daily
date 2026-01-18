import ReactMarkdown from 'react-markdown'

export function MarkdownRenderer({ content }) {
  if (!content) return null

  // Remove YAML frontmatter if present
  let cleanContent = content
  if (content.startsWith('---\n')) {
    const endIndex = content.indexOf('\n---\n', 4)
    if (endIndex !== -1) {
      cleanContent = content.slice(endIndex + 5)
    }
  }

  return (
    <ReactMarkdown
      components={{
        // Custom heading rendering
        h1: ({ children }) => (
          <h1 className="text-2xl font-bold text-orange-400 mb-4 text-balance">
            {children}
          </h1>
        ),
        h2: ({ children }) => (
          <h2 className="text-xl font-semibold text-orange-300 mt-6 mb-3 text-balance">
            {children}
          </h2>
        ),
        h3: ({ children }) => (
          <h3 className="text-lg font-medium text-gray-200 mt-4 mb-2 text-balance">
            {children}
          </h3>
        ),
        // Paragraphs
        p: ({ children }) => (
          <p className="text-gray-300 mb-3 leading-relaxed text-pretty">
            {children}
          </p>
        ),
        // Lists
        ul: ({ children }) => (
          <ul className="list-disc list-inside mb-3 space-y-1">
            {children}
          </ul>
        ),
        ol: ({ children }) => (
          <ol className="list-decimal list-inside mb-3 space-y-1">
            {children}
          </ol>
        ),
        li: ({ children }) => (
          <li className="text-gray-300">{children}</li>
        ),
        // Code
        code: ({ inline, children, className }) => {
          if (inline) {
            return (
              <code className="bg-daily-dark px-1.5 py-0.5 rounded text-orange-300 text-sm">
                {children}
              </code>
            )
          }
          return (
            <code className={className}>{children}</code>
          )
        },
        pre: ({ children }) => (
          <pre className="bg-daily-dark p-4 rounded-lg overflow-x-auto mb-4 text-sm">
            {children}
          </pre>
        ),
        // Links
        a: ({ href, children }) => (
          <a
            href={href}
            className="text-orange-400 hover:text-orange-300 underline"
            target="_blank"
            rel="noopener noreferrer"
          >
            {children}
          </a>
        ),
        // Blockquote
        blockquote: ({ children }) => (
          <blockquote className="border-l-4 border-orange-500 pl-4 italic text-gray-400 my-4">
            {children}
          </blockquote>
        ),
        // Strong/Bold
        strong: ({ children }) => (
          <strong className="font-semibold text-gray-200">{children}</strong>
        ),
        // Emphasis/Italic
        em: ({ children }) => (
          <em className="italic text-gray-300">{children}</em>
        ),
      }}
    >
      {cleanContent}
    </ReactMarkdown>
  )
}
