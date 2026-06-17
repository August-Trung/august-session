# V0.2.0 Module Dependency Graph: August Session

This document describes the structural relationships, boundaries, and information flows of August Session v0.2.0.

---

## 1. Rust Backend Hierarchy (src-tauri)

v0.2.0 adds two new modules: `explorer.rs` (COM-based folder extraction) and `title_parser.rs` (title-based file path extraction). Both are called by `window_manager.rs` during enumeration.

```
       ┌──────────────────────────────────────────────────────────────┐
       │                        main.rs                               │
       │  (App Lifecycle, Tray setup, Command Router)                 │
       └──────┬──────────────┬──────────────────┬─────────────────────┘
              │              │                  │
              ▼              ▼                  ▼
     ┌──────────────┐ ┌──────────────┐ ┌────────────────────────┐
     │    db.rs     │ │  capture.rs  │ │   window_manager.rs    │
     │ (SQLite Init,│ │ (Screenshot  │ │ (Win32 Enum, Relaunch  │
     │  CRUD Ops)   │ │  Generation) │ │  & Smart Positioning)  │
     └──────────────┘ └──────────────┘ └──────┬─────────────────┘
                                              │
                                ┌─────────────┴─────────────┐
                                │                           │
                                ▼                           ▼
                     ┌──────────────────┐       ┌──────────────────┐
                     │   explorer.rs    │       │  title_parser.rs │
                     │ (COM API for     │       │ (Title parsing   │
                     │  Explorer folder │       │  for Office/PDF/ │
                     │  path extraction)│       │  Text/VS Code)   │
                     └──────────────────┘       └──────────────────┘
```

### Module Responsibilities

| Module | v0.1.0 | v0.2.0 Changes |
|---|---|---|
| `main.rs` | Command router, tray setup | Add `mod explorer;` and `mod title_parser;` |
| `db.rs` | CRUD operations | No changes |
| `capture.rs` | Screenshot capture | No changes |
| `tray.rs` | System tray menu | No changes |
| `window_manager.rs` | Enumerate, close, restore | Call extraction functions, smart restore with `launch_args` |
| `explorer.rs` | (new) | Extract folder path from File Explorer via COM |
| `title_parser.rs` | (new) | Extract file path from window title for Office/PDF/Text/VS Code |

---

## 2. Vue Frontend Hierarchy (src)

No frontend changes in v0.2.0. The component tree remains identical to v0.1.0.

```
                   ┌───────────────────────┐
                   │        App.vue        │
                   │ (Shell View Switcher) │
                   └───────────┬───────────┘
                               │
                ┌──────────────┴──────────────┐
                ▼                             ▼
     ┌─────────────────────┐       ┌─────────────────────┐
     │   PauseOverlay.vue  │       │   ResumeView.vue    │
     │   (New Context Text)│       │  (Active Workspace) │
     └─────────────────────┘       └──────────┬──────────┘
                                              │
                                              ▼
                                   ┌─────────────────────┐
                                   │  RememberList.vue   │
                                   │ (Historical Journal)│
                                   └─────────────────────┘
```

---

## 3. Cross-Layer Communications (Frontend ↔ Backend)

No new IPC commands are added. The existing `save_moment`, `get_moments`, `delete_moment`, and `restore_moment` commands continue to work. The `launch_args` data flows entirely within the Rust backend — it is captured during `save_moment` and consumed during `restore_moment`.

```
┌─────────────────────────────────────────────────────────────────┐
│                          VUE FRONTEND                           │
│                                                                 │
│   PauseOverlay.vue                 ResumeView.vue               │
└─────────┬─────────────────────────────────┬─────────────────────┘
          │                                 │
          │ tauri::invoke("save_moment")    │ tauri::invoke("restore_moment")
          ▼                                 ▼
┌─────────────────────────────────────────────────────────────────┐
│                       TAURI IPC BRIDGE                          │
└─────────┬─────────────────────────────────┬─────────────────────┘
          │                                 │
          ▼                                 ▼
┌─────────────────────────────────────────────────────────────────┐
│                          RUST BACKEND                           │
│                                                                 │
│   save_moment:                    restore_moment:               │
│   main.rs                         main.rs                       │
│     ├─► window_manager.rs           └─► window_manager.rs       │
│     │     ├─► explorer.rs                (ShellExecuteW with    │
│     │     └─► title_parser.rs             launch_args)          │
│     ├─► capture.rs                                              │
│     └─► db.rs                                                   │
└─────────────────────────────────────────────────────────────────┘
```

---

## 4. Key Product Flows

### The Pause Flow (v0.2.0 Enhanced)
Triggered by global hotkey. Now extracts application content context in addition to window coordinates.

```
[User Hotkey] ──► [main.rs] ──► [capture.rs] (Screenshot captured)
                     │
                     ├────────► [window_manager.rs] (Windows enumerated)
                     │                │
                     │                ├──► [explorer.rs] (Explorer folder paths extracted)
                     │                └──► [title_parser.rs] (Document file paths extracted)
                     │
                     ▼
             [PauseOverlay.vue] (Displays input form)
                     │
                     ▼
             [User submits note]
                     │
                     ▼
             [main.rs] ──► [db.rs] (Moment with launch_args committed to SQLite)
```

### The Restore Flow (v0.2.0 Enhanced)
Reopens applications with their content, not just their executables.

```
[User clicks "Reopen My Desk"] ──► [ResumeView.vue]
                                           │
                                           ▼
                                tauri::invoke("restore_moment")
                                           │
                                           ▼
                                  [window_manager.rs]
                                           │
                                           ├─► launch_args = Some(path)?
                                           │     YES: ShellExecute(exe, path) → App opens with content
                                           │     NO:  ShellExecute(exe, null) → App opens empty (v0.1.0 fallback)
                                           │
                                           ├─► Browser dedup check
                                           │     Already launched? Skip.
                                           │     First instance? Launch with --restore-last-session
                                           │
                                           └─► SetWindowPos() (App repositioned)
```

---

## 5. Data Flow: launch_args Lifecycle

```
[Enumeration]  ──► explorer.rs / title_parser.rs ──► launch_args = Some("C:\...\file.docx")
       │
       ▼
[Serialization] ──► serde_json::to_string() ──► JSON includes "launch_args": "C:\...\file.docx"
       │
       ▼
[Database]      ──► SQLite `windows` column stores the full JSON array
       │
       ▼
[Deserialization] ──► serde_json::from_str() ──► WindowInfo { launch_args: Some("C:\...\file.docx") }
       │
       ▼
[Restoration]   ──► ShellExecuteW(exe_path, launch_args) ──► App opens with file
```

---

## 6. Architectural Boundaries & Isolation Rules
- **No Direct Frontend DB Queries:** Unchanged from v0.1.0.
- **Stateless Extraction Modules:** Both `explorer.rs` and `title_parser.rs` are pure functions that receive inputs and return outputs. They do not maintain internal state.
- **Graceful Fallback:** If any extraction function fails, it returns `None` and the window falls back to v0.1.0 behavior. No crash, no error popup.
- **No New IPC Commands:** The `launch_args` field is internal to the Rust backend. The frontend does not need to know about it.
- **Backward Compatible Serialization:** Old Moments without `launch_args` deserialize to `None` via `#[serde(default)]`.
