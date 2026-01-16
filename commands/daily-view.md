---
description: "View today's daily archive or a specific date"
argument-hint: "[--date YYYY-MM-DD] [--list] [--summary-only]"
allowed-tools: ["Bash(daily view:*)"]
---

# Daily View Command

View the daily context archive.

## Usage

To view today's archive:
```bash
daily view
```

To view a specific date:
```bash
daily view --date $ARGUMENTS
```

To list all sessions:
```bash
daily view --list
```

To view only the daily summary:
```bash
daily view --summary-only
```

Display the output to the user in a readable format.
