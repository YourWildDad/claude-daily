import { Outlet } from 'react-router-dom'
import { ArchiveTree } from './ArchiveTree'

export function ArchiveLayout() {
  return (
    <div className="flex h-[calc(100vh-4rem)] overflow-hidden">
      {/* Left sidebar - Archive tree */}
      <aside className="w-80 shrink-0 border-r border-gray-800 bg-black flex flex-col">
        <div className="p-4 border-b border-gray-800">
          <h2 className="text-lg font-semibold text-orange-400">Archives</h2>
        </div>
        <div className="flex-1 overflow-hidden">
          <ArchiveTree />
        </div>
      </aside>

      {/* Right content area */}
      <main className="flex-1 overflow-y-auto">
        <Outlet />
      </main>
    </div>
  )
}
