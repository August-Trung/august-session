# Module Dependency Graph: August Session

This document describes the structural relationships, boundaries, and information flows of August Session v0.1.0.

---

## 1. Rust Backend Hierarchy (src-tauri)

The Rust backend is structured with flat modules. The entry point is `main.rs`, which orchestrates modules and exposes Tauri commands.

```
       ┌──────────────────────────────────────────────────┐
       │                   main.rs                        │
       │  (App Lifecycle, Tray setup, Command Router)     │
       └─────────┬───────────────┬─────────────────┬──────┘
                 │               │                 │
                 ▼               ▼                 ▼
        ┌──────────────┐ ┌──────────────┐ ┌────────────────┐
        │    db.rs     │ │  capture.rs  │ │window_manager.s│
        │ (SQLite Init,│ │ (Screenshot  │ │ (Win32 Enum,  │
        │  CRUD Ops)   │ │  Generation) │ │  Relaunch &    │
        │              │ │              │ │  Positioning)  │
        └──────────────┘ └──────────────┘ └────────────────┘
```

---

## 2. Vue Frontend Hierarchy (src)

The frontend uses standard Vue components. It is intentionally simple, avoiding deep component nesting.

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

All cross-layer boundary calls pass through Tauri's IPC Bridge. Frontend calls commands; backend invokes window visibility handlers.

```
┌─────────────────────────────────────────────────────────────────┐
│                          VUE FRONTEND                           │
│                                                                 │
│   PauseOverlay.vue                 ResumeView.vue               │
└─────────┬─────────────────────────────────┬─────────────────────┘
          │                                 │
          │ tauri::invoke("save_moment")    │ tauri::invoke("get_moments")
          ▼                                 ▼
┌─────────────────────────────────────────────────────────────────┐
│                       TAURI IPC BRIDGE                          │
└─────────┬─────────────────────────────────┬─────────────────────┘
          │                                 │
          ▼                                 ▼
┌─────────────────────────────────────────────────────────────────┐
│                          RUST BACKEND                           │
│                                                                 │
│   main.rs ──► db.rs                main.rs ──► db.rs            │
└─────────────────────────────────────────────────────────────────┘
```

---

## 4. Key Product Flows

### The Pause Flow
Triggered by global hotkey. Captures desk configuration, prompts user, and saves record.

```
[User Hotkey] ──► [main.rs] ──► [capture.rs] (Screenshot captured)
                     │
                     ├────────► [window_manager.rs] (Windows enumerated)
                     │
                     ▼
             [PauseOverlay.vue] (Displays input form)
                     │
                     ▼
             [User submits note]
                     │
                     ▼
             [main.rs] ──► [db.rs] (Moments row committed to SQLite)
```

### The Resume Flow
Renders the saved state to trigger memory recognition.

```
[App Launched] ──► [ResumeView.vue] ──► tauri::invoke("get_latest_moment")
                                                    │
                                                    ▼
   [ResumeView.vue] ◄── [JSON data & WebP path] ◄── [db.rs]
          │
          ├─► Renders large text of your_words
          └─► Renders file:/// path to local screenshot WebP
```

### The Restore Flow
Reopens and moves applications.

```
[User clicks "Reopen My Desk"] ──► [ResumeView.vue]
                                           │
                                           ▼
                                tauri::invoke("restore_moment")
                                           │
                                           ▼
                                  [window_manager.rs]
                                           │
                                           ├─► shell::openPath() (App launched)
                                           └─► SetWindowPos() (App repositioned)
```

---

## 5. Architectural Boundaries & Isolation Rules
- **No Direct Frontend DB Queries:** The Vue application never talks directly to SQLite. Database operations are strictly wrapped in Rust functions annotated with `#[tauri::command]`.
- **Stateless Modules:** Both `capture.rs` and `window_manager.rs` do not maintain internal state. They receive inputs, execute OS commands, and return outcomes to `main.rs`.
- **Filesystem Access:** Frontend references screenshots using Tauri's safe asset protocol (`https://asset.localhost/` or `asset://`). No arbitrary file loading from the frontend.
