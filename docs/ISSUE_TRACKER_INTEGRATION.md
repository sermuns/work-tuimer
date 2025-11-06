# Issue Tracker Integration

WorkTimer supports automatic ticket detection from task names and browser integration for **any** issue tracker (JIRA, Linear, GitHub Issues, GitLab, Azure DevOps, etc.). **This feature is completely optional** - the application works perfectly without any configuration.

## Table of Contents

- [Setup](#setup)
- [Configuration Examples](#configuration-examples)
- [Usage](#usage)
- [Ticket Detection](#ticket-detection)
- [Multiple Tracker Support](#multiple-tracker-support)
- [Supported Platforms](#supported-platforms)

## Setup

**Note**: If you don't create a config file, the integration feature will be hidden (no `T`/`L` keybindings, no ticket badges). The app works perfectly without it.

To enable the integration, create a configuration file at the appropriate location for your platform:

- **Linux/macOS**: `~/.config/work-tuimer/config.toml` (or `$XDG_CONFIG_HOME/work-tuimer/config.toml` if set)
- **Windows**: `%APPDATA%\work-tuimer\config.toml`

## Configuration Examples

### Example: JIRA tracker

```toml
[integrations]
default_tracker = "my-jira"  # Default tracker when pattern is ambiguous

[integrations.trackers.my-jira]
enabled = true
base_url = "https://your-company.atlassian.net"
ticket_patterns = ["^PROJ-\\d+$", "^WORK-\\d+$"]  # Regex to match your tickets
browse_url = "{base_url}/browse/{ticket}"
worklog_url = "{base_url}/browse/{ticket}?focusedWorklogId=-1"
```

### Example: GitHub Issues tracker

```toml
[integrations]
default_tracker = "github"

[integrations.trackers.github]
enabled = true
base_url = "https://github.com/yourorg/yourrepo"
ticket_patterns = ["^#\\d+$"]  # Matches #123
browse_url = "{base_url}/issues/{ticket}"
worklog_url = ""  # GitHub doesn't have worklogs
```

### Example: Multiple trackers (JIRA + Linear + GitHub)

```toml
[integrations]
default_tracker = "work-jira"  # Fallback when patterns overlap

[integrations.trackers.work-jira]
enabled = true
base_url = "https://company.atlassian.net"
ticket_patterns = ["^PROJ-\\d+$", "^WORK-\\d+$"]  # Company JIRA projects
browse_url = "{base_url}/browse/{ticket}"
worklog_url = "{base_url}/browse/{ticket}?focusedWorklogId=-1"

[integrations.trackers.team-linear]
enabled = true
base_url = "https://linear.app/your-team"
ticket_patterns = ["^ENG-\\d+$", "^DESIGN-\\d+$"]  # Linear team patterns
browse_url = "{base_url}/issue/{ticket}"
worklog_url = ""

[integrations.trackers.oss-github]
enabled = true
base_url = "https://github.com/myorg/myrepo"
ticket_patterns = ["^#\\d+$"]  # GitHub issue numbers
browse_url = "{base_url}/issues/{ticket}"
worklog_url = ""
```

## Usage

### 1. Include ticket IDs in task names

When creating or editing tasks, include the ticket ID in the name:
- JIRA: `"PROJ-123: Fix login bug"`
- Linear: `"ENG-456: Add dark mode"`
- GitHub: `"#789: Update documentation"`

### 2. Visual indicator

Tasks with detected tickets show a badge with a ticket icon: `🎫 Task Name [PROJ-123]`

### 3. Open ticket in browser

Press `T` (capital T) while a task with a detected ticket (🎫 icon visible) is selected to open the ticket in your default browser

### 4. Open worklog URL

Press `L` (capital L) while a task with a detected ticket (🎫 icon visible) is selected to open the worklog URL (if configured). Useful for JIRA users to quickly jump to the worklog entry form for a ticket

**Note**: The `T` and `L` keybindings only appear in the footer and only work when:
- Integrations are configured in `config.toml`
- The selected task has a ticket ID that matches one of your `ticket_patterns` (indicated by the 🎫 icon)

## Ticket Detection

The system uses regex patterns defined in `ticket_patterns` to detect ticket IDs from task names. Common patterns:
- **JIRA/Linear**: `^[A-Z]+-\\d+$` matches `PROJ-123`, `ENG-456`
- **GitHub Issues**: `^#\\d+$` matches `#123`, `#456`
- **Custom**: Define any regex pattern that matches your tracker's ticket format

Tickets are detected automatically from task names at runtime (no data model changes required).

## Multiple Tracker Support

**Multiple Tracker Support**: The app automatically detects which tracker to use based on the `ticket_patterns` regex:
- Each tracker is checked in order until a pattern matches the ticket ID
- If a ticket matches multiple patterns, the **first matching tracker** in the config is used
- If **no pattern matches**, it falls back to the `default_tracker` (useful for catch-all scenarios or tickets that don't follow a strict pattern)
- You can name your trackers anything you want (e.g., `work-jira`, `my-company-tracker`, `team-issues`)

**Best Practice**: Define specific patterns for each tracker to avoid conflicts:
- ✅ Good: JIRA uses `^PROJ-\\d+$`, GitHub uses `^#\\d+$`, Linear uses `^ENG-\\d+$` (distinct patterns)
- ❌ Avoid: JIRA uses `^[A-Z]+-\\d+$`, Linear uses `^[A-Z]+-\\d+$` (overlapping - first one wins)

## Supported Platforms

- **macOS**: Uses `open` command
- **Linux**: Uses `xdg-open` command  
- **Windows**: Uses `cmd /C start` command
