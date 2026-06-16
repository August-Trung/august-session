# V0.1.0 Code Order: August Session

This document establishes the optimal coding sequence for a solo developer implementing the August Session v0.1.0 MVP.

---

## Coding Sequence Strategy

The sequence follows a **high-risk-first, vertical-slice approach**. Instead of completing the entire frontend first or writing a complex Rust engine in isolation, we build working, end-to-end features incrementally.

```
       ┌────────────────────────────────────────────────────────┐
       │ 1. Set up Tauri + SQLite (Data baseline)               │
       └───────────────────────────┬────────────────────────────┘
                                   ▼
       ┌────────────────────────────────────────────────────────┐
       │ 2. Build Rust Window/Capture APIs (High technical risk)│
       └───────────────────────────┬────────────────────────────┘
                                   ▼
       ┌────────────────────────────────────────────────────────┐
       │ 3. Create Vue frontend components (User experience)    │
       └───────────────────────────┬────────────────────────────┘
                                   ▼
       ┌────────────────────────────────────────────────────────┐
       │ 4. Implement window restore & polish (End-to-end flow) │
       └────────────────────────────────────────────────────────┘
```

---

## Detailed Development Stages

### Stage 1: Data Baseline & Shell
1. **Initialize Tauri App** ([M1-T1](file:///d:/Study/Projects/000-Desktop_App/August-Session/docs/V010_IMPLEMENTATION_TASKS.md#task-m1-t1-project-initialization)): Establishes the workspace and ensures the framework runs.
2. **Database Initialization** ([M1-T2](file:///d:/Study/Projects/000-Desktop_App/August-Session/docs/V010_IMPLEMENTATION_TASKS.md#task-m1-t2-sqlite-database-setup-schema-integration)): Creates the storage layer on launch. All subsequent capture logic relies on this.
3. **Tray Menu & Hotkey** ([M1-T4](file:///d:/Study/Projects/000-Desktop_App/August-Session/docs/V010_IMPLEMENTATION_TASKS.md#task-m1-t4-system-tray-lifecycle-and-global-hotkey-register)): Sets up the persistent background structure and entry point for the Pause action.

*Checkpoint 1:* The app starts minimized to the system tray. Pressing `Ctrl+Shift+P` triggers a logged message. The SQLite file is created correctly in app data.

---

### Stage 2: Capture Engine (High Risk)
1. **Win32 Window Enumeration** ([M2-T2](file:///d:/Study/Projects/000-Desktop_App/August-Session/docs/V010_IMPLEMENTATION_TASKS.md#task-m2-t2-win32-window-enumeration)): The most technically challenging part. Needs to run reliably on Windows without permissions failures.
2. **Desktop Screenshot Capture** ([M2-T1](file:///d:/Study/Projects/000-Desktop_App/August-Session/docs/V010_IMPLEMENTATION_TASKS.md#task-m2-t1-desktop-screenshot-capture)): Captures the screen layout.

*Checkpoint 2:* Pressing the hotkey captures a screenshot to the local filesystem and logs a JSON list of open application paths, titles, and window coordinates.

---

### Stage 3: UX Views & Commands
1. **Pause Overlay Component** ([M3-T1](file:///d:/Study/Projects/000-Desktop_App/August-Session/docs/V010_IMPLEMENTATION_TASKS.md#task-m3-t1-pause-overlay-interface)): Integrates the text input overlay.
2. **Database Save Command** ([M4-T1](file:///d:/Study/Projects/000-Desktop_App/August-Session/docs/V010_IMPLEMENTATION_TASKS.md#task-m4-t1-database-integration-service)): Links the overlay input to database insertion.
3. **Resume View & History** ([M3-T2](file:///d:/Study/Projects/000-Desktop_App/August-Session/docs/V010_IMPLEMENTATION_TASKS.md#task-m3-t2-resume-screen-ui), [M3-T3](file:///d:/Study/Projects/000-Desktop_App/August-Session/docs/V010_IMPLEMENTATION_TASKS.md#task-m3-t3-remember-journal-list)): Implements the core dashboard and history retrieval UI.

*Checkpoint 3:* Pressing `Ctrl+Shift+P` opens the overlay. Typing a note and hitting Enter saves the Moment and screenshot. Opening the main app displays your note, the image, and a list of previous moments.

---

### Stage 4: Restoration & Polish
1. **Window Restoration Service** ([M4-T2](file:///d:/Study/Projects/000-Desktop_App/August-Session/docs/V010_IMPLEMENTATION_TASKS.md#task-m4-t2-window-restoration-relaunch-service)): Implements process launching and positioning.
2. **Graceful Closing Logic** ([M4-T3](file:///d:/Study/Projects/000-Desktop_App/August-Session/docs/V010_IMPLEMENTATION_TASKS.md#task-m4-t3-workspace-window-closure-behavior)): Closes target applications.
3. **E2E Validation**: Final verification of application stability, database state integrity, and cleanup.

*Checkpoint 4:* Full end-to-end test. Opening multiple applications, pausing, writing a note, selecting "Close everything", and later resuming the setup in under 5 seconds.

---

## Why This Sequence Minimizes Risk
1. **High-Risk-First:** Window enumeration and coordinate calculation are the most OS-dependent and error-prone components. Building them before the UI ensures we don't design an interface for data we can't reliably capture.
2. **Local Data First:** Establishing SQLite schema ensures that the data structures are well-defined and locked down before building frontend Vue components.
3. **Incremental Runnable State:** The application remains runnable at each checkpoint, allowing immediate manual debugging of OS integrations.
