use crate::archive::SessionArchive;

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
  "summary": "2-3 sentence overview of what was accomplished",
  "decisions": "Key decisions made and their rationale (markdown list format)",
  "learnings": "Key learnings from this session (markdown list format)",
  "skill_hints": "Potential reusable skills identified (markdown format with name, description, trigger)"
}}
```

Focus on:
1. What was the main goal and was it achieved?
2. What key decisions were made and why?
3. What patterns could become reusable skills/commands?

Output ONLY the JSON block, no additional text."#
        )
    }

    /// Generate prompt for daily summary
    pub fn daily_summary(sessions_json: &str, date: &str) -> String {
        format!(
            r#"You are analyzing all Claude Code sessions from {date}. Generate a comprehensive daily summary.

Sessions completed today (JSON format):
{sessions_json}

Generate a thoughtful daily summary with these sections:

1. **Overview**: 2-3 sentence synthesis of the day's work
2. **Session Details**: Brief description of each session's outcome
3. **Key Insights**: Deep learnings that connect multiple sessions or represent important discoveries
4. **Skills**: Potential skills to extract (name, description, when to use)
5. **Commands**: Potential commands to extract (name, description, use case)
6. **Reflections**: Thoughts on work patterns, productivity, challenges
7. **Tomorrow's Focus**: Based on incomplete tasks or emerging priorities

Output format (JSON):
```json
{{
  "overview": "...",
  "session_details": "markdown formatted list of sessions",
  "insights": "markdown list of insights",
  "skills": "markdown formatted skill suggestions",
  "commands": "markdown formatted command suggestions",
  "reflections": "thoughtful reflection paragraph",
  "tomorrow_focus": "suggestions for tomorrow"
}}
```

Output ONLY the JSON block."#
        )
    }

    /// Generate prompt for skill extraction
    pub fn extract_skill(session_summary: &str, skill_hint: Option<&str>) -> String {
        let hint = skill_hint.unwrap_or("Based on patterns in the session");

        format!(
            r#"Generate a complete SKILL.md file for Claude Code based on this session.

Session Summary:
{session_summary}

Skill Hint: {hint}

Generate a complete skill file that:
1. Has clear trigger conditions (when should Claude use this skill)
2. Provides step-by-step workflow
3. Includes best practices from the session
4. Is immediately usable

Output the complete SKILL.md content following this format:
```markdown
---
name: skill-name-kebab-case
description: "Brief description"
---

# Skill Name

[Description of what this skill does]

## Trigger

Use this skill when: [trigger conditions]

## Workflow

[Step-by-step instructions]

## Best Practices

[Key practices to follow]

## Examples

[Usage examples]
```

Output ONLY the markdown content."#
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
        );

        assert!(prompt.contains("2026-01-16"));
    }
}
