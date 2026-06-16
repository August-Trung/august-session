# V0.1.0 File Structure: August Session

This document details the file and folder layout of the August Session v0.1.0 codebase.

---

## Workspace Layout

```
August-Session/
├── package.json
├── tsconfig.json
├── vite.config.ts
├── docs/                       -- Product Documentation
│   ├── PRODUCT_VISION.md
│   ├── ARCHITECTURE.md
│   ├── MVP_IMPLEMENTATION_PLAN.md
│   ├── IMPLEMENTATION_TASKS.md
│   ├── CODE_ORDER.md
│   ├── MODULE_DEPENDENCY_GRAPH.md
│   └── FILE_STRUCTURE.md
│
├── src/                        -- Frontend Application (Vue 3)
│   ├── main.ts                 -- Entrypoint
│   ├── App.vue                 -- Layout Switcher
│   ├── assets/                 -- Static Icons/CSS
│   │   └── main.css            -- Global CSS rules
│   ├── plugins/
│   │   └── vuetify.ts          -- Vuetify 3 Setup
│   ├── views/
│   │   ├── PauseOverlay.vue    -- Overlay Input Layout
│   │   └── ResumeView.vue      -- Main Resume Screen & Remember History
│   └── components/
│       └── RememberList.vue    -- Scrollable Moments Sub-component
│
└── src-tauri/                  -- Backend Application (Rust + Tauri)
    ├── Cargo.toml              -- Rust Package Manifest
    ├── tauri.conf.json         -- Tauri Config File
    └── src/
        ├── main.rs             -- Application Entrypoint & IPC Comm Router
        ├── db.rs               -- SQLite schema initialization & database CRUD queries
        ├── capture.rs          -- Desktop screenshot generation logic
        ├── tray.rs             -- System tray menu construction & behaviors
        └── window_manager.rs   -- Win32 API interactions (Enumerating, closing, restoring)
```

---

## File Overview

### New Files Created in v0.1.0

#### Frontend (`src/`)
- `src/plugins/vuetify.ts`: Configures theme and icons for the UI components.
- `src/views/PauseOverlay.vue`: The simple screen focused on capturing the note before stopping work.
- `src/views/ResumeView.vue`: The landing page when opening August Session. Handles loading and display of the latest Moment.
- `src/components/RememberList.vue`: Lists historical entries directly beneath the main layout.

#### Backend (`src-tauri/`)
- `src-tauri/src/db.rs`: Sets up the SQLite database and executes raw SQL for Moment records.
- `src-tauri/src/capture.rs`: Handles the screenshot logic.
- `src-tauri/src/window_manager.rs`: Uses Windows native APIs to enumerate open windows and reposition them during restoration.
- `src-tauri/src/tray.rs`: Handles system tray menu initialization and interaction.

---

## Design Rationale

1. **Shallow Folder Depth:** We explicitly avoid deep folders like `src/core/services/impl/db/`. In the backend, modules are placed directly under `src-tauri/src/`. In the frontend, files are organized into flat folders: `components`, `views`, and `plugins`.
2. **No Single-File Directories:** We do not create directories that contain only one file. For instance, `RememberList.vue` is kept inside `components/` alongside any other UI widgets we may add.
3. **No Speculative Abstractions:** There are no `momentRepository.ts` or `sqliteConnectionManager.rs` files. Database code is written directly in `db.rs`. Window management operations are contained directly in `window_manager.rs`.
4. **Naming Conventions:**
   - Frontend files use `PascalCase` for Vue files (`PauseOverlay.vue`), and `camelCase` for TypeScript files (`vuetify.ts`).
   - Backend files use `snake_case` (`window_manager.rs`) to align with Rust design guidelines.
