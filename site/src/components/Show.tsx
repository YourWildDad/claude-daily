import { motion } from 'framer-motion';

const sections = [
  {
    icon: '📅',
    title: 'Daily Overview',
    desc: 'AI auto-generated summary of the day\'s work, distilling key accomplishments and activities from all sessions.',
  },
  {
    icon: '💡',
    title: 'Key Insights',
    desc: 'Important discoveries, patterns, and lessons learned extracted automatically from your coding sessions.',
  },
  {
    icon: '🎯',
    title: "Tomorrow's Focus",
    desc: 'AI-suggested action items and priorities for the next day based on today\'s progress and findings.',
  },
  {
    icon: '📝',
    title: 'Sessions',
    desc: 'List of individual work sessions with clickable cards to view detailed session summaries.',
  },
];

const features = [
  {
    icon: '🤖',
    title: 'AI Summarization',
    desc: 'Claude analyzes your session transcripts and generates structured summaries automatically.',
  },
  {
    icon: '📊',
    title: 'Web Dashboard',
    desc: 'Browse archives, view summaries, and monitor background jobs through a modern web interface.',
  },
  {
    icon: '⚡',
    title: 'Real-time Updates',
    desc: 'WebSocket-powered live updates show summarization progress as it happens.',
  },
  {
    icon: '🔍',
    title: 'Easy Navigation',
    desc: 'Navigate by date, search sessions, and quickly jump to specific work contexts.',
  },
];

export default function Show() {
  return (
    <div className="min-h-screen bg-[#0a0a0a] pt-20 pb-16">
      {/* Hero */}
      <section className="px-6 py-16">
        <div className="max-w-4xl mx-auto text-center">
          <motion.h1
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            className="text-4xl md:text-5xl font-bold text-gray-100 mb-4"
          >
            Daily Show
          </motion.h1>
          <motion.p
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.1 }}
            className="text-lg text-gray-400"
          >
            See how Daily captures and presents your coding journey
          </motion.p>
        </div>
      </section>

      {/* Screenshot */}
      <section className="px-6 pb-16">
        <div className="max-w-5xl mx-auto">
          <motion.div
            initial={{ opacity: 0, scale: 0.98 }}
            animate={{ opacity: 1, scale: 1 }}
            transition={{ delay: 0.2 }}
            className="rounded-xl overflow-hidden border border-orange-500/30 shadow-2xl shadow-orange-500/10"
          >
            <img
              src="/daily-show.png"
              alt="Daily Dashboard Screenshot"
              className="w-full"
            />
          </motion.div>
        </div>
      </section>

      {/* UI Sections Explanation */}
      <section className="px-6 pb-16">
        <div className="max-w-4xl mx-auto">
          <h2 className="text-2xl font-bold text-gray-100 mb-6">Interface Overview</h2>
          <div className="grid md:grid-cols-2 gap-4">
            {sections.map((section, idx) => (
              <motion.div
                key={section.title}
                initial={{ opacity: 0, x: -20 }}
                animate={{ opacity: 1, x: 0 }}
                transition={{ delay: 0.3 + idx * 0.1 }}
                className="bg-[#1a1a1a] rounded-xl p-5 border border-orange-500/20 flex gap-4"
              >
                <div className="text-3xl">{section.icon}</div>
                <div>
                  <h3 className="font-semibold text-orange-400 mb-1">{section.title}</h3>
                  <p className="text-sm text-gray-400">{section.desc}</p>
                </div>
              </motion.div>
            ))}
          </div>
        </div>
      </section>

      {/* Dashboard Features */}
      <section className="px-6 pb-16">
        <div className="max-w-4xl mx-auto">
          <h2 className="text-2xl font-bold text-gray-100 mb-6">Dashboard Features</h2>
          <div className="grid sm:grid-cols-2 lg:grid-cols-4 gap-4">
            {features.map((feature, idx) => (
              <motion.div
                key={feature.title}
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ delay: 0.5 + idx * 0.1 }}
                className="bg-[#1a1a1a] rounded-xl p-4 border border-orange-500/20"
              >
                <div className="text-2xl mb-2">{feature.icon}</div>
                <h3 className="font-medium text-gray-200 mb-1">{feature.title}</h3>
                <p className="text-xs text-gray-500">{feature.desc}</p>
              </motion.div>
            ))}
          </div>
        </div>
      </section>

      {/* How It Works */}
      <section className="px-6 pb-16">
        <div className="max-w-4xl mx-auto">
          <h2 className="text-2xl font-bold text-gray-100 mb-6">How It Works</h2>
          <div className="bg-[#1a1a1a] rounded-xl p-6 border border-orange-500/20">
            <div className="space-y-4">
              <div className="flex items-start gap-4">
                <span className="w-8 h-8 rounded-full bg-orange-500 text-black text-sm font-bold flex items-center justify-center shrink-0">1</span>
                <div>
                  <h3 className="font-medium text-gray-200">Session Ends</h3>
                  <p className="text-sm text-gray-500">When you finish a Claude Code session, a hook triggers automatically.</p>
                </div>
              </div>
              <div className="flex items-start gap-4">
                <span className="w-8 h-8 rounded-full bg-orange-500 text-black text-sm font-bold flex items-center justify-center shrink-0">2</span>
                <div>
                  <h3 className="font-medium text-gray-200">Background Processing</h3>
                  <p className="text-sm text-gray-500">A background job invokes Claude to analyze the session transcript.</p>
                </div>
              </div>
              <div className="flex items-start gap-4">
                <span className="w-8 h-8 rounded-full bg-orange-500 text-black text-sm font-bold flex items-center justify-center shrink-0">3</span>
                <div>
                  <h3 className="font-medium text-gray-200">Summary Generated</h3>
                  <p className="text-sm text-gray-500">AI generates a structured markdown summary with overview, insights, and next steps.</p>
                </div>
              </div>
              <div className="flex items-start gap-4">
                <span className="w-8 h-8 rounded-full bg-orange-500 text-black text-sm font-bold flex items-center justify-center shrink-0">4</span>
                <div>
                  <h3 className="font-medium text-gray-200">Browse in Dashboard</h3>
                  <p className="text-sm text-gray-500">Launch <code className="text-orange-400">daily show</code> to view all archives in the web dashboard.</p>
                </div>
              </div>
            </div>
          </div>
        </div>
      </section>

      {/* Quick Commands */}
      <section className="px-6 pb-16">
        <div className="max-w-4xl mx-auto">
          <h2 className="text-2xl font-bold text-gray-100 mb-6">Quick Commands</h2>
          <div className="bg-[#1a1a1a] rounded-xl p-6 border border-orange-500/20 font-mono text-sm space-y-3">
            <div>
              <span className="text-gray-500"># Start the dashboard</span>
            </div>
            <div>
              <code className="text-orange-300">daily show</code>
            </div>
            <div className="pt-2">
              <span className="text-gray-500"># View today's archive in terminal</span>
            </div>
            <div>
              <code className="text-orange-300">daily view</code>
            </div>
            <div className="pt-2">
              <span className="text-gray-500"># Generate daily digest</span>
            </div>
            <div>
              <code className="text-orange-300">daily digest</code>
            </div>
          </div>
        </div>
      </section>
    </div>
  );
}
