use chrono::Timelike;

/// Prompts for Claude CLI summarization
pub struct Prompts;

impl Prompts {
    /// Generate prompt for session summarization
    pub fn session_summary(transcript_text: &str, cwd: &str, git_info: Option<&str>) -> String {
        let git_str = git_info.unwrap_or("N/A");

        format!(
            r#"You are analyzing a Claude Code session transcript. Generate a comprehensive summary in JSON format.

Context:
- Working Directory: {cwd}
- Git Branch: {git_str}

Transcript:
{transcript_text}

Generate a JSON response with this exact structure:
```json
{{
  "topic": "Short kebab-case topic for filename (2-4 words, e.g., 'fix-auth-bug', 'add-dark-mode', 'refactor-api')",
  "summary": "2-3 sentence overview including CONCRETE RESULTS (answers found, solutions implemented, code written). Never just describe the action - always include what was produced or discovered.",
  "decisions": "Key decisions made and their rationale (markdown list format)",
  "learnings": "Key learnings from this session (markdown list format)",
  "skill_hints": "Potential reusable skills (only if passes quality gate, see below)"
}}
```

## Skill Quality Gate (沉淀三问)
Only suggest skills that pass ALL three criteria:
1. **踩过坑吗？** Did debugging, trial-and-error, or non-obvious discovery occur?
2. **下次还会遇到吗？** Is this a recurring problem, not a one-time edge case?
3. **能说清楚吗？** Can the solution be clearly described and verified?

For skill_hints format (only if quality gate passes):
```
- **[skill-name]**: [what it solves]
  - Trigger: [error message or symptom]
  - Why: [root cause]
```

If no skills pass the quality gate, set skill_hints to "None identified in this session."

Output ONLY the JSON block, no additional text."#
        )
    }

    /// Generate prompt for daily summary
    pub fn daily_summary(
        sessions_json: &str,
        date: &str,
        existing_summary: Option<&str>,
    ) -> String {
        let now = chrono::Local::now();
        let current_time = now.format("%H:%M").to_string();
        let current_hour = now.hour();

        // Determine current period for context
        let current_period = match current_hour {
            0..=5 => "凌晨",
            6..=11 => "早上",
            12..=17 => "下午",
            _ => "晚上",
        };

        // Check if this is a regenerate scenario (no new sessions but existing summary)
        let is_regenerate = sessions_json.trim() == "[]" && existing_summary.is_some();

        let existing_section = if let Some(existing) = existing_summary {
            if is_regenerate {
                // Regenerate mode: rewrite the existing summary with improved format
                format!(
                    "
## REGENERATE MODE

You are regenerating an existing daily summary. The original content is below.
Your task is to REWRITE it with better structure and time accuracy, NOT to add new content.

Original daily.md content:
```
{existing}
```

IMPORTANT for regeneration:
- Extract session information from the Sessions section in the original content
- Parse timestamps from session names to determine ACTUAL time periods
- Rewrite the overview to reflect the ACTUAL time distribution
- Preserve all insights, reflections, and tomorrow's focus but improve clarity
- Do NOT fabricate sessions or content that wasn't in the original
",
                    existing = existing
                )
            } else {
                // Incremental mode: merge existing with new sessions
                format!(
                    r#"
## Existing Daily Summary (from previous digest)

The following content was generated from earlier sessions today. You MUST preserve and integrate this content with the new sessions:

```
{existing}
```

IMPORTANT: Merge the existing summary with the new sessions. Do NOT discard existing content.
- Combine overviews into a comprehensive day summary
- Append new session details to existing ones
- Merge insights, skills, commands (avoid duplicates)
- Update reflections to cover the full day
- Revise tomorrow's focus based on all work done
"#,
                    existing = existing
                )
            }
        } else {
            String::new()
        };

        // Skip sessions section in regenerate mode since it's empty
        let sessions_section = if is_regenerate {
            String::new()
        } else {
            format!("## Sessions (JSON format):\n{}", sessions_json)
        };

        format!(
            r#"You are analyzing Claude Code sessions from {date}. Generate a daily summary.

## Time Context
- Current time: {current_time} ({current_period})
- Session names contain timestamps: e.g., "21_03-fix-bug" means 21:03 (晚上), "09_30-add-feature" means 09:30 (早上)
- Time periods: 凌晨 (00:00-05:59), 早上 (06:00-11:59), 下午 (12:00-17:59), 晚上 (18:00-23:59)

CRITICAL: Parse the actual timestamps from session names to determine time periods. NEVER fabricate times like "上午...下午..." if all sessions are in the evening.
{existing_section}
{sessions_section}

## Your Task

Generate a summary that answers: "今天问了什么？聊了什么？有什么收获？接下来要做什么？"

### Output Structure

1. **Overview**: 2-3 sentences describing what happened today. Use ACTUAL time periods based on session timestamps (e.g., "今晚主要在..." if all sessions are after 18:00).

2. **Sessions**: List each session with:
   - Session name with emoji indicating type (🔧 fix, 📚 research, 💬 chat, 🎨 UI, 📋 plan)
   - One-line description of what was discussed/accomplished

3. **Key Insights**: Valuable learnings worth remembering. Focus on:
   - Technical discoveries (root causes, solutions found)
   - Patterns observed
   - Connections between topics

4. **Skills & Commands Identified**: Reusable patterns that could become skills or commands (if any, otherwise say "暂未发现")

5. **Reflections**: Brief thoughts on work patterns, what went well, what could improve

6. **Tomorrow's Focus**: High-value TODOs based on:
   - Unfinished tasks
   - Problems discovered but not yet solved
   - Natural next steps

Output format (JSON):
```json
{{
  "overview": "...",
  "session_details": "markdown formatted list",
  "insights": "markdown list of insights",
  "skills": "markdown formatted skill suggestions (or '暂未发现')",
  "commands": "markdown formatted command suggestions (or '暂未发现')",
  "reflections": "thoughtful reflection paragraph",
  "tomorrow_focus": "prioritized suggestions"
}}
```

Output ONLY the JSON block. Ensure all strings in JSON are properly escaped (especially quotes and newlines)."#,
            current_time = current_time,
            current_period = current_period,
            existing_section = existing_section,
            sessions_section = sessions_section,
            date = date
        )
    }

    /// Generate prompt for skill extraction
    pub fn extract_skill(session_summary: &str, skill_hint: Option<&str>) -> String {
        let hint = skill_hint.unwrap_or("Based on patterns in the session");
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();

        format!(
            r#"You are extracting a reusable skill from a Claude Code session.

## Quality Gate - Answer these three questions first:

1. **踩过坑吗？** Was there trial-and-error, debugging, or a non-obvious discovery?
2. **下次还会遇到吗？** Is this a recurring problem, not a one-time edge case?
3. **能说清楚吗？** Can the solution be clearly described and verified?

If ANY answer is NO, respond with:
```
NOT_EXTRACTABLE: [reason]
```

If ALL answers are YES, generate the skill.

## Session Summary:
{session_summary}

Skill Hint: {hint}

## Output Format:

```markdown
---
name: skill-name-kebab-case
description: "Retrieval-optimized: include error messages, symptoms, or how user might describe the problem. Max 100 tokens."
origin: "{today}/session-name"
confidence: verified
---

# Skill Name

Brief description of what this skill solves.

## When to Use

Trigger this skill when you encounter:
- [Exact error message or symptom, e.g., "ECONNREFUSED on port 3000"]
- [How user might describe it, e.g., "my dev server won't start"]
- [Related scenarios]

## Root Cause

Why does this problem happen? Understanding the cause prevents future issues.

## Solution

Step-by-step resolution:

1. [First step]
2. [Second step]
...

## Verification

How to confirm the problem is solved:
- [Check command or expected output]
```

Output ONLY the markdown content (or NOT_EXTRACTABLE message)."#,
            today = today
        )
    }

    /// Generate prompt for command extraction
    pub fn extract_command(session_summary: &str, command_hint: Option<&str>) -> String {
        let hint = command_hint.unwrap_or("Based on patterns in the session");

        format!(
            r#"Generate a complete slash command file for Claude Code based on this session.

Session Summary:
{session_summary}

Command Hint: {hint}

Generate a command file that:
1. Has a clear description
2. Explains when to use it
3. Provides instructions for Claude to follow
4. Is immediately usable as a /command

Output the complete command markdown following this format:
```markdown
---
description: "Brief description of what this command does"
---

# Command Name

[When to use this command]

## Instructions

[Instructions for Claude to follow when this command is invoked]
```

Output ONLY the markdown content."#
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_summary_prompt() {
        let prompt = Prompts::session_summary(
            "User: Help me fix a bug\nAssistant: I'll help you.",
            "/home/user/project",
            Some("main"),
        );

        assert!(prompt.contains("Working Directory: /home/user/project"));
        assert!(prompt.contains("Git Branch: main"));
    }

    #[test]
    fn test_daily_summary_prompt() {
        let prompt = Prompts::daily_summary(
            r#"[{"title": "test", "summary": "test summary"}]"#,
            "2026-01-16",
            None,
        );

        assert!(prompt.contains("2026-01-16"));
    }

    #[test]
    fn test_daily_summary_prompt_with_existing() {
        let prompt = Prompts::daily_summary(
            r#"[{"title": "new", "summary": "new summary"}]"#,
            "2026-01-16",
            Some("Previous overview content"),
        );

        assert!(prompt.contains("2026-01-16"));
        assert!(prompt.contains("Previous overview content"));
        assert!(prompt.contains("Existing Daily Summary"));
    }
}
