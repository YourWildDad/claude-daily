---
description: "Extract and generate a command from daily archive insights"
argument-hint: "[--session NAME] [--output PATH]"
allowed-tools: ["Bash(daily extract-command:*)", "Write(**/commands/**/*.md)"]
---

# Extract Command

Extract a reusable command from today's session insights.

## Workflow

1. First, list today's sessions and command suggestions:
```bash
daily view --list
```

2. Review the command hints from the daily summary:
```bash
daily view --summary-only
```

3. Extract the command:
```bash
daily extract-command $ARGUMENTS
```

4. The command will be generated and saved. Review the output and offer to:
   - Install to user commands: `~/.claude/commands/`
   - Install to project commands: `.claude/commands/`
   - Modify the generated command

Ask the user where they want to install the command and make any requested modifications.
