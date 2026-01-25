/**
 * Example data for template preview
 */

export const EXAMPLE_DATA = {
  session_summary: {
    transcript: `User: Help me implement a dark mode toggle feature
Assistant: I'll help you add a dark mode toggle. Let me first check your current theme setup...
[After reading files]
I've added the DarkModeToggle component and integrated it with your theme context.

User: Great! Can you also add keyboard shortcut support?
Assistant: Sure, I'll add Cmd+D to toggle dark mode...`,
    cwd: '/Users/example/my-project',
    git_branch: 'feature/dark-mode',
    language: 'en',
  },

  daily_summary: {
    date: '2026-01-25',
    current_time: '18:30',
    current_period: 'evening',
    periods_desc: 'morning (before 12:00), afternoon (12:00-18:00), evening (after 18:00)',
    existing_section: `## Overview

Today focused on implementing UI improvements and fixing authentication bugs.

## Sessions

### 1. Dark Mode Implementation (09:30 - 10:45)
- Added theme context and toggle component
- Integrated with localStorage for persistence
- Fixed contrast issues in code blocks`,
    sessions_section: `### 2. Authentication Bug Fix (14:20 - 15:10)
- Fixed token refresh logic
- Added error handling for expired sessions
- Updated tests`,
    sessions_json: JSON.stringify([
      {
        title: 'Dark Mode Implementation',
        time: '09:30',
        summary: 'Added dark mode toggle with theme context',
      },
      {
        title: 'Authentication Bug Fix',
        time: '14:20',
        summary: 'Fixed token refresh and session handling',
      },
    ], null, 2),
    language: 'en',
  },

  skill_extract: {
    session_content: `# Dark Mode Implementation

## Summary
Implemented a complete dark mode toggle system with theme context, localStorage persistence, and keyboard shortcuts.

## Key Decisions
- Used React Context API for theme state management
- Chose CSS variables for dynamic theming
- Added Cmd+D keyboard shortcut for quick toggle

## Code Changes
- Created ThemeContext and ThemeProvider
- Added DarkModeToggle component
- Updated global CSS with theme variables
- Implemented useKeyboardShortcut hook`,
    skill_hint: 'A reusable pattern for implementing dark mode in React applications',
    today: '2026-01-25',
    language: 'en',
  },

  command_extract: {
    session_content: `# Authentication Bug Fix

## Summary
Fixed critical authentication bug where token refresh was failing, causing users to be logged out unexpectedly.

## Key Decisions
- Implemented exponential backoff for retry logic
- Added comprehensive error handling
- Updated tests to cover edge cases

## Code Changes
- Modified TokenRefreshService to handle network errors
- Added retry mechanism with exponential backoff
- Updated AuthContext to clear state on auth failure
- Added 15 new test cases for auth flows`,
    command_hint: 'A slash command to fix common authentication issues following this debugging pattern',
    language: 'en',
  },
}
