# Timer Sessions

WorkTimer includes a powerful built-in timer system for real-time time tracking. Sessions allow you to track time as you work, with automatic updates, pause/resume support, and seamless integration between the TUI and CLI.

## Table of Contents

- [What are Sessions?](#what-are-sessions)
- [TUI Usage](#tui-usage)
- [CLI Usage](#cli-usage)
- [Session Features](#session-features)
- [Common Workflows](#common-workflows)
- [Data Persistence](#data-persistence)

## What are Sessions?

A **session** is an active timer that tracks the time you spend on a specific task. When you start a session, WorkTimer records:
- Task name and optional description
- Start time
- Elapsed time (updated in real-time)
- Pause/resume history
- Final duration when stopped

Sessions are designed to eliminate manual time entry - just start a timer when you begin work and stop it when you're done. The end time is automatically set to the current time when you stop.

## TUI Usage

### Starting a Session

There are two ways to start a session in the TUI:

1. **From an existing work record**: Select a record and press `S`
   - The session will update that record's end time when stopped
   - Useful for extending existing work entries

2. **From the TUI timer interface**: Press `S` when no record is selected
   - Creates a new work record when stopped
   - Start time is set to when you started the timer

### Session Controls

| Key | Action |
|-----|--------|
| `S` | Start/Stop session on selected record |
| `P` | Pause/Resume active session |

### Visual Indicators

When a session is active, you'll see:

1. **Timer Status Bar** (top of screen):
   ```
   ⏱ Running: Task Name | 1h 23m 45s | Status: Running
   ```
   Shows task name, elapsed time (H:MM:SS format), and current status (Running/Paused)

2. **Record Highlighting**:
   - Active session records are highlighted with a ⏱ icon
   - Makes it easy to see which record is being tracked

### Session States

- **Running**: Time is actively being tracked
- **Paused**: Timer is paused, paused duration is tracked separately
- **Stopped**: Session has ended, time is saved to the work record

## CLI Usage

The CLI provides full session control from the command line, perfect for scripting or quick operations without opening the TUI.

### Starting a Session

```bash
# Start a basic session
work-tuimer session start "My Task"

# Start with a description
work-tuimer session start "Bug Fix" -d "Fixing authentication issue"
```

Output:
```
✓ Session started
  Task: My Task
  Description: Optional description
  Started at: 14:30:45
```

### Checking Session Status

```bash
work-tuimer session status
```

Output:
```
⏱ Session Status
  Task: My Task
  Status: Running
  Elapsed: 1h 23m 45s
  Started at: 14:30:45
  Description: Optional description
```

### Pausing and Resuming

```bash
# Pause the active session
work-tuimer session pause
```

Output:
```
⏸ Session paused
  Task: My Task
  Elapsed: 0m 45s
```

```bash
# Resume the paused session
work-tuimer session resume
```

Output:
```
▶ Session resumed
  Task: My Task
  Total elapsed (before pause): 0m 45s
```

### Stopping a Session

```bash
work-tuimer session stop
```

Output:
```
✓ Session stopped
  Task: My Task
  Duration: 1h 23m 45s
  Started at: 14:30:45
  Ended at: 15:54:30
```

### Error Handling

If you try to control a session when none is running:
```bash
$ work-tuimer session stop
Error: No session is running
```

If you try to start a session when one is already running:
```bash
$ work-tuimer session start "Another Task"
Error: A timer is already running
```

## Session Features

### Automatic Time Updates

When you stop a session, the end time is automatically set to the current time. No need to manually enter when you finished - just stop the timer and WorkTimer records it for you.

### Pause Support

Sessions can be paused and resumed:
- **Elapsed time**: Only counts active time (paused time is excluded)
- **Paused duration**: Tracked separately for reference
- **Multiple pauses**: Pause and resume as many times as needed

### Cross-Session Persistence

Sessions survive application restarts:
- Close the TUI while a session is running
- Your session continues in the background
- Reopen the TUI or use the CLI to check status or stop

The session state is saved to `~/.local/share/work-tuimer/active_timer.json`

### Cross-Date Support

Start a session on a record from any day:
- Navigate to a past day in the TUI
- Start a session on an old record
- When stopped, the end time updates correctly
- Works for future dates too (for planning)

### CLI and TUI Integration

Sessions work seamlessly across both interfaces:
- Start a session in the CLI, pause it in the TUI
- Start in the TUI, check status in the CLI
- Changes in one interface are immediately visible in the other

**Auto-reload feature**: The TUI checks for external file changes every 500ms, so CLI-created records appear automatically without manual refresh.

## Common Workflows

### Workflow 1: Simple Session

```bash
# Start working
work-tuimer session start "Write documentation"

# ... work on your task ...

# Stop when done
work-tuimer session stop
```

### Workflow 2: Session with Breaks

```bash
# Start working
work-tuimer session start "Code review"

# ... work for a while ...

# Take a break
work-tuimer session pause

# ... break time ...

# Resume work
work-tuimer session resume

# ... finish up ...

# Stop when done
work-tuimer session stop
```

### Workflow 3: TUI + CLI Hybrid

```bash
# Start in CLI before opening TUI
work-tuimer session start "Morning standup prep"

# Open TUI to view full day
work-tuimer

# Continue working in TUI, see session status at top
# Press P to pause, S to stop, or let it run

# Later, check status from CLI
work-tuimer session status

# Stop from CLI when done
work-tuimer session stop
```

### Workflow 4: Updating Existing Records

In the TUI:
1. Navigate to a work record you want to extend
2. Press `S` to start a session
3. Work continues...
4. Press `S` again to stop

The record's end time updates to when you stopped, and duration recalculates automatically.

### Workflow 5: Quick Status Checks

```bash
# Quick check if anything is running
work-tuimer session status

# If nothing running, start new task
work-tuimer session start "New task"
```

## Data Persistence

### Active Session Storage

Active sessions are stored in:
- **Linux/macOS**: `~/.local/share/work-tuimer/active_timer.json`
- **Windows**: `%APPDATA%\work-tuimer\active_timer.json`
- **Fallback**: `./data/active_timer.json`

### Session State Format

```json
{
  "task_name": "My Task",
  "description": "Optional description",
  "start_time": "2025-11-12T14:30:45.123456789Z",
  "status": "Running",
  "paused_duration_secs": 0,
  "source_record_id": 1,
  "date": "2025-11-12"
}
```

### Work Record Integration

When a session stops, it creates or updates a work record in the daily file:

```json
{
  "date": "2025-11-12",
  "work_records": [
    {
      "id": 1,
      "name": "My Task",
      "start": "14:30",
      "end": "15:54",
      "total_minutes": 84,
      "description": "Optional description"
    }
  ]
}
```

### File Location Priority

Daily work records are saved to (checked in order):
1. `~/.local/share/work-tuimer/YYYY-MM-DD.json`
2. `./data/YYYY-MM-DD.json` (fallback)

## Tips and Best Practices

1. **Use descriptive task names**: Make it easy to identify what you worked on
2. **Add descriptions for context**: Helpful when reviewing your time later
3. **Pause during interruptions**: Get accurate time tracking by pausing during breaks
4. **Check status regularly**: Use `work-tuimer session status` to stay aware of running sessions
5. **Stop sessions promptly**: Don't forget to stop when switching tasks
6. **Use CLI for quick starts**: Start sessions from your terminal without opening the TUI
7. **Leverage auto-reload**: Work in CLI and TUI simultaneously without conflicts

## Troubleshooting

### Session not showing in TUI

- The TUI auto-reloads every 500ms, wait a moment
- If still not visible, restart the TUI

### Lost session after restart

- Sessions are saved to `active_timer.json` - check if the file exists
- If the file was deleted, the session cannot be recovered

### Wrong end time on stopped session

- End time is set to when you stopped, not when the timer started
- If you forgot to stop earlier, manually edit the end time in the TUI

### CLI and TUI showing different data

- The TUI caches data and auto-reloads every 500ms
- Wait a moment or restart the TUI to see latest changes
