# Architecture

*The simplest system that delivers the emotional experience.*

---

## Principle

If a component doesn't directly contribute to the moment of "ah, right," it doesn't belong. Every technical decision optimizes for shipping, not for elegance.

---

## System Overview

```
                    ┌───────────────────┐
                    │   Global Hotkey   │
                    │   (Pause trigger) │
                    └────────┬──────────┘
                             │
                             ▼
┌──────────────────────────────────────────────────┐
│                August Session                     │
│               (Electron app)                      │
│                                                   │
│  ┌─────────────────────────────────────────────┐  │
│  │              Main Process                   │  │
│  │                                             │  │
│  │  ┌──────────┐ ┌──────────┐ ┌─────────────┐ │  │
│  │  │Screenshot│ │ Window   │ │  Reopen     │ │  │
│  │  │  Capture  │ │ Enumer.  │ │  Service    │ │  │
│  │  └──────────┘ └──────────┘ └─────────────┘ │  │
│  │                                             │  │
│  │  ┌────────────────────────────────────────┐ │  │
│  │  │  Store (SQLite + filesystem)           │ │  │
│  │  └────────────────────────────────────────┘ │  │
│  └─────────────────────────────────────────────┘  │
│                                                   │
│  ┌─────────────────────────────────────────────┐  │
│  │            Renderer Process                 │  │
│  │                                             │  │
│  │  ┌──────────────┐  ┌────────────────────┐   │  │
│  │  │ Pause Overlay│  │  Resume + Remember │   │  │
│  │  └──────────────┘  └────────────────────┘   │  │
│  └─────────────────────────────────────────────┘  │
│                                                   │
│  ┌─────────────────────────────────────────────┐  │
│  │              System Tray                    │  │
│  │         (persistent, minimal)               │  │
│  └─────────────────────────────────────────────┘  │
│                                                   │
└──────────────────────────────────────────────────┘
```

---

## Components

### System Tray (persistent)

The only thing that runs continuously. Sits in the system tray. Consumes near-zero resources. Does nothing except:

- Listen for the global hotkey
- Provide a tray icon with "Pause" and "Open August" options
- Keep the app alive in the background

No monitoring. No polling. No activity tracking. It waits.

### Screenshot Capture

When the user presses the Pause hotkey, captures a screenshot of the entire desktop. Saves it as a compressed image (JPEG or WebP, ~200KB) to the local filesystem.

Uses Electron's `desktopCapturer` API. One call. One image. Done.

### Window Enumeration

When the user presses the Pause hotkey, enumerates all visible windows on the desktop. For each window, records:

- Application name
- Window title
- Executable path (for relaunching)
- Position (x, y) and size (width, height)

On Windows: uses native Win32 APIs (`EnumWindows`, `GetWindowText`, `GetWindowRect`, `GetWindowThreadProcessId`) via a native Node addon or `ffi-napi`.

The result is a simple JSON array. No interpretation. No classification. Just what's there.

### Pause Overlay

A frameless, always-on-top Electron `BrowserWindow` that appears when the user presses the Pause hotkey. Displays:

1. The prompt: "What should you remember?"
2. A text input field
3. A "Done" button (also triggered by Enter)
4. A "Close everything" checkbox (optional)

After the user submits:

- The Moment is saved (words + screenshot + windows + timestamp)
- If "close everything" is checked, all captured windows are closed
- The overlay disappears

The overlay is minimal. It should feel like a gentle interruption, not a dialog box. It appears, you type, you leave.

### Resume Window

The main application window. Shown when the user opens August (from tray, from desktop shortcut, or from the global "open" hotkey).

Displays:

1. **Your words** — the most recent Moment's text, large and centered
2. **Your desk** — the screenshot, below the words
3. **"Reopen My Desk"** button — below the screenshot
4. **Earlier moments** — a scrollable list below, showing timestamp and words for each past Moment

The visual hierarchy is strict: words first, screenshot second, button third. This is the entire UI for Resume and Remember — they share one screen.

### Reopen Service

When the user clicks "Reopen My Desk," iterates through the Moment's window list and:

1. Launches each application by its executable path
2. After a brief delay (to let applications start), positions each window at its recorded coordinates and size

This is best-effort, not perfect. Applications may open to their default state rather than the exact previous state. That's acceptable — the mental reconstruction already happened when the user read their words and saw the screenshot. The reopening is a convenience, not the core.

On Windows: uses `child_process.exec` or `shell.openPath` for launching, and Win32 APIs (`SetWindowPos`, `MoveWindow`) for positioning.

### Store

**SQLite** — one database file, one table.

**Filesystem** — one directory of screenshot images.

Both stored in the application's user data directory (e.g., `%APPDATA%/august-session/`).

No migrations framework. The schema is simple enough that a single `CREATE TABLE IF NOT EXISTS` statement suffices.

---

## Data Model

### SQLite Schema

```sql
CREATE TABLE IF NOT EXISTS moments (
  id          TEXT PRIMARY KEY,
  words       TEXT,
  windows     TEXT NOT NULL,  -- JSON array
  screenshot  TEXT NOT NULL,  -- filename (relative to screenshots dir)
  created_at  TEXT NOT NULL   -- ISO 8601 timestamp
);

CREATE INDEX IF NOT EXISTS idx_moments_created_at ON moments(created_at DESC);
```

### Moment Structure

```
{
  "id": "a1b2c3d4",
  "words": "Finish last 3 slides. Print worksheets for Period 3.",
  "windows": [
    {
      "app": "Google Chrome",
      "title": "Mitosis - YouTube",
      "exe": "C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe",
      "x": 0, "y": 0, "w": 960, "h": 1080
    },
    {
      "app": "Google Chrome",
      "title": "Cell Division Lesson - Google Slides",
      "exe": "C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe",
      "x": 960, "y": 0, "w": 960, "h": 1080
    },
    {
      "app": "Explorer",
      "title": "Lab Photos",
      "exe": "C:\\Windows\\explorer.exe",
      "x": 0, "y": 540, "w": 480, "h": 540
    }
  ],
  "screenshot": "2026-06-17T01-38-00.webp",
  "created_at": "2026-06-17T01:38:00+07:00"
}
```

### Filesystem Layout

```
%APPDATA%/august-session/
├── august.db                    -- SQLite database
└── screenshots/
    ├── 2026-06-17T01-38-00.webp
    ├── 2026-06-16T17-22-00.webp
    └── ...
```

---

## Process Architecture

### Lifecycle

```
App launch
  │
  ├─ Start tray icon
  ├─ Register global hotkey
  └─ If launched by user click (not login startup):
       └─ Show Resume window
  
On Pause hotkey:
  │
  ├─ Capture screenshot → save to filesystem
  ├─ Enumerate windows → hold in memory
  ├─ Show Pause overlay
  │    └─ User types words, presses Enter
  ├─ Save Moment to SQLite
  ├─ If "close everything": close captured windows
  └─ Hide overlay
  
On tray icon click / Open hotkey:
  │
  └─ Show Resume window
       ├─ Load most recent Moment from SQLite
       ├─ Display words + screenshot + button
       └─ Load list of past Moments (timestamp + words)
  
On "Reopen My Desk":
  │
  ├─ For each window in Moment:
  │    ├─ Launch executable
  │    └─ Position window (after delay)
  └─ Optionally minimize/hide August window
```

### What Runs When

| State | What's running | Resources |
|---|---|---|
| Background (normal) | Tray icon + hotkey listener | ~30 MB RAM, 0% CPU |
| Pause (momentary) | Overlay window + capture | Brief spike, then idle |
| Resume (on demand) | Main window | Normal Electron window |
| Closed | Nothing | Nothing |

---

## Technology Choices

| Component | Technology | Why |
|---|---|---|
| Application shell | Electron | Cross-platform desktop access, tray, global hotkeys, window management. Mature, well-documented. |
| UI | React + TypeScript | Straightforward component model. Two views (Pause overlay, Resume window). |
| Styling | Vanilla CSS | No framework needed for two views. Custom properties for theming. |
| Database | better-sqlite3 | Synchronous, fast, zero-config, single-file. Perfect for a one-table schema. |
| Screenshots | Electron desktopCapturer | Built into Electron. No external dependency. |
| Window enumeration | Native addon (node-ffi-napi or custom N-API) | Required for Win32 window APIs. Small, focused native module. |
| Window positioning | Same native addon | Same Win32 APIs for SetWindowPos. |
| IDs | nanoid | Tiny, fast, URL-safe unique IDs. |
| Image format | WebP | Good compression, wide support, keeps screenshots small. |

### What Is Deliberately Not Chosen

| Omission | Why |
|---|---|
| No React Router | Two views can be toggled with state. No routing needed. |
| No state management library | React useState and useEffect are sufficient. |
| No ORM | One table. Raw SQL is clearer. |
| No build pipeline beyond Electron's | electron-builder handles packaging. |
| No testing framework initially | Ship first. Test what survives. |
| No analytics | We don't watch users. |
| No auto-updater initially | Manual releases until the product stabilizes. |

---

## Size Budget

The app should feel instant. Targets:

- **Cold start to tray**: < 2 seconds
- **Hotkey to overlay visible**: < 500ms
- **Tray click to Resume visible**: < 500ms
- **Screenshot capture + save**: < 1 second
- **Window enumeration**: < 200ms
- **Reopen all windows**: < 5 seconds (dependent on application launch times)
- **Database query (latest Moment)**: < 10ms
- **Installed size**: < 150 MB (Electron baseline ~120 MB)
- **Background memory**: < 50 MB

---

## What Is Not Architected

The following are explicitly out of scope for this architecture. They do not have database columns, API surfaces, config options, or abstraction layers waiting for them.

- Cloud sync
- User accounts
- Browser extension communication
- Editor integration protocols
- Plugin systems
- Activity monitoring
- Background polling
- Notification systems
- Search indexing
- Data export

If any of these are ever needed, they will be designed from scratch at that time. Premature abstraction is a cost, not an investment.
