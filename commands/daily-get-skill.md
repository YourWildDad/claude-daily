---
description: "Extract and generate a skill from daily archive insights"
argument-hint: "[--session NAME] [--output PATH]"
allowed-tools: ["Bash(daily extract-skill:*)", "Write(**/skills/**/*.md)"]
---

# Extract Skill Command

Extract a reusable skill from today's session insights.

## Workflow

1. First, list today's sessions and skill suggestions:
```bash
daily view --list
```

2. Review the skill hints from the daily summary:
```bash
daily view --summary-only
```

3. Extract the skill:
```bash
daily extract-skill $ARGUMENTS
```

4. The skill will be generated and saved. Review the output and offer to:
   - Install to user skills: `~/.claude/skills/`
   - Install to project skills: `.claude/skills/`
   - Modify the generated skill

Ask the user where they want to install the skill and make any requested modifications.
