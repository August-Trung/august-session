# V0.2.0 File Structure: August Session

This document details the file and folder layout of the August Session v0.2.0 codebase.

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
│   ├── PROJECT_PLAN.md
│   ├── MVP_IMPLEMENTATION_PLAN.md
│   ├── IMPLEMENTATION_TASKS.md
│   ├── CODE_ORDER.md
│   ├── MODULE_DEPENDENCY_GRAPH.md
│   ├── FILE_STRUCTURE.md
│   ├── COMPETITOR_ANALYSIS.md
│   ├── FUTURE_ECOSYSTEM.md
│   ├── RUN_TASK.md
│   ├── V010_IMPLEMENTATION_PLAN.md
│   ├── V010_IMPLEMENTATION_TASKS.md
│   ├── V010_CODE_ORDER.md
│   ├── V010_MODULE_DEPENDENCY_GRAPH.md
│   ├── V010_FILE_STRUCTURE.md
│   ├── V020_IMPLEMENTATION_PLAN.md
│   ├── V020_IMPLEMENTATION_TASKS.md
│   ├── V020_CODE_ORDER.md
│   ├── V020_MODULE_DEPENDENCY_GRAPH.md
│   └── V020_FILE_STRUCTURE.md
│
├── src/                        -- Frontend Application (Vue 3) [UNCHANGED]
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
    ├── Cargo.toml              -- Rust Package Manifest (updated with `windows` crate)
    ├── tauri.conf.json         -- Tauri Config File
    └── src/
        ├── main.rs             -- Application Entrypoint & IPC Comm Router
        ├── db.rs               -- SQLite schema initialization & database CRUD queries
        ├── capture.rs          -- Desktop screenshot generation logic
        ├── tray.rs             -- System tray menu construction & behaviors
        ├── window_manager.rs   -- Win32 API interactions (Enumerating, closing, smart restoring)
        ├── explorer.rs         -- [NEW] File Explorer folder path extraction via COM API
        └── title_parser.rs     -- [NEW] Window title parsing for Office/PDF/Text/VS Code
```

---

## File Overview

### New Files Created in v0.2.0

#### Backend (`src-tauri/`)
- `src-tauri/src/explorer.rs`: Uses Windows COM automation (`IShellWindows`) to extract the currently displayed folder path from open File Explorer windows. Returns the absolute directory path for each Explorer window handle.
- `src-tauri/src/title_parser.rs`: Parses window titles of known applications (Word, Excel, PowerPoint, Adobe Reader, Foxit, SumatraPDF, Notepad, Notepad++, VS Code, image viewers) to extract the filename, then searches common user directories to resolve the absolute file path.

### Modified Files in v0.2.0

#### Backend (`src-tauri/`)
- `src-tauri/src/window_manager.rs`: Extended `WindowInfo` struct with `launch_args: Option<String>`. Updated enumeration callback to call `explorer.rs` and `title_parser.rs` for argument extraction. Updated `restore_windows` to pass `launch_args` to `ShellExecuteW`. Added browser deduplication logic.
- `src-tauri/src/main.rs`: Added `mod explorer;` and `mod title_parser;` declarations.
- `src-tauri/Cargo.toml`: Added `windows` crate dependency for COM API access.

### Unchanged Files

#### Frontend (`src/`)
No frontend files are modified in v0.2.0. The UI remains identical.

#### Backend (`src-tauri/`)
- `src-tauri/src/db.rs`: No changes. The `windows` column already stores JSON, and the new `launch_args` field is part of the serialized `WindowInfo`.
- `src-tauri/src/capture.rs`: No changes.
- `src-tauri/src/tray.rs`: No changes.

---

## Design Rationale

1. **Shallow Folder Depth:** Maintained from v0.1.0. New modules `explorer.rs` and `title_parser.rs` are placed directly under `src-tauri/src/`, not in subdirectories.
2. **No Single-File Directories:** No new directories are created.
3. **No Speculative Abstractions:** There is no `IExtractor` trait, no plugin system, no `ExtractorRegistry`. Each module is a focused collection of functions.
4. **Separation of Concerns:** `explorer.rs` handles COM-specific Windows Explorer extraction. `title_parser.rs` handles generic title-based extraction. `window_manager.rs` orchestrates both.
5. **Naming Conventions:** Unchanged from v0.1.0.
   - Backend files use `snake_case` (`explorer.rs`, `title_parser.rs`).
   - Frontend files use `PascalCase` for Vue files and `camelCase` for TypeScript files.
