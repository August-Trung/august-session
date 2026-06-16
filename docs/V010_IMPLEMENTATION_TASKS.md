# V0.1.0 Implementation Tasks: August Session

This document details the granular, step-by-step programming tasks required to implement the v0.1.0 MVP of August Session.

---

## Milestone 1: Local Data & Shell (Tauri + Rust + SQLite)

### Task M1-T1: Project Initialization
- **Task ID:** M1-T1
- **Goal:** Initialize the Tauri project workspace with Vue 3, TypeScript, and Vuetify 3.
- **Files to Create:**
  - `src/plugins/vuetify.ts`
- **Files to Modify:**
  - `package.json`
  - `src/main.ts`
  - `src-tauri/Cargo.toml`
  - `src-tauri/tauri.conf.json`
- **Dependencies:** None
- **Complexity:** Small
- **Risk Level:** Low
- **Implementation Notes:** Ensure `tauri.conf.json` is configured with `systemTray` settings and the window starts hidden or minimized to tray depending on launch configurations. Use standard Vite integrations for Vue 3 + Vuetify 3.
- **Manual Test Steps:** Run `npm run tauri dev`. Verify the app launches and displays a blank window with Vuetify components loaded correctly.
- **Acceptance Criteria:** Standard Tauri frontend displays Vue 3 component page with Vuetify styling without errors.

---

### Task M1-T2: SQLite Database Setup & Schema Integration
- **Task ID:** M1-T2
- **Goal:** Set up SQLite database initialization on Rust startup and create the `moments` table.
- **Files to Create:**
  - `src-tauri/src/db.rs`
- **Files to Modify:**
  - `src-tauri/src/main.rs`
  - `src-tauri/Cargo.toml`
- **Dependencies:** M1-T1
- **Complexity:** Medium
- **Risk Level:** Low
- **Implementation Notes:** Use `rusqlite` crate. On startup, initialize the SQLite file in the user app data directory under `august-session/august.db`. Create the `moments` table if it does not exist. Create the index on `created_at`.
- **Manual Test Steps:** Launch the app. Check `%APPDATA%/august-session/` (on Windows) to confirm `august.db` file is created.
- **Acceptance Criteria:** `august.db` is initialized and table structure matches the defined schema.

---

### Task M1-T3: Filesystem Snapshot Directory Setup
- **Task ID:** M1-T3
- **Goal:** Initialize the screenshot storage directory on startup.
- **Files to Create:** None
- **Files to Modify:**
  - `src-tauri/src/main.rs`
- **Dependencies:** M1-T1
- **Complexity:** Small
- **Risk Level:** Low
- **Implementation Notes:** Use Rust's `std::fs::create_dir_all` to create the `screenshots` folder inside the app data directory alongside the database.
- **Manual Test Steps:** Launch the app. Confirm that `%APPDATA%/august-session/screenshots/` folder is present.
- **Acceptance Criteria:** Directory exists and is writeable by the application process.

---

### Task M1-T4: System Tray Lifecycle and Global Hotkey Register
- **Task ID:** M1-T4
- **Goal:** Implement the system tray behavior (Show, Hide, Exit) and register a global shortcut.
- **Files to Create:**
  - `src-tauri/src/tray.rs`
- **Files to Modify:**
  - `src-tauri/src/main.rs`
  - `src-tauri/Cargo.toml`
- **Dependencies:** M1-T1
- **Complexity:** Medium
- **Risk Level:** Medium
- **Implementation Notes:** Register system tray menu using Tauri's tray API. Bind `Ctrl+Shift+P` globally. When hotkey is pressed, trigger the Pause overlay window display.
- **Manual Test Steps:** Minimize window to tray. Right-click tray icon to verify menu options. Press `Ctrl+Shift+P` globally while in another app and check if it triggers a log output.
- **Acceptance Criteria:** Tray icon functions, and pressing the hotkey logs the trigger event correctly without crashing.

---

## Milestone 2: The Capture Engine (Screenshot & Win32)

### Task M2-T1: Desktop Screenshot Capture
- **Task ID:** M2-T1
- **Goal:** Capture the full desktop screen layout and save it as a WebP image.
- **Files to Create:**
  - `src-tauri/src/capture.rs`
- **Files to Modify:**
  - `src-tauri/src/main.rs`
  - `src-tauri/Cargo.toml`
- **Dependencies:** M1-T3, M1-T4
- **Complexity:** Medium
- **Risk Level:** Medium
- **Implementation Notes:** Use the `screenshot-rs` or `image` + `scrap` crates, or launch a lightweight screenshot helper call. Compress the image to WebP or high-quality JPEG and write it to the `screenshots/` directory with a unique timestamped filename.
- **Manual Test Steps:** Trigger a manual command that takes a screenshot. Confirm the file is saved in the screenshots folder and is readable.
- **Acceptance Criteria:** A valid WebP/JPEG screenshot file is created representing the current display layout.

---

### Task M2-T2: Win32 Window Enumeration
- **Task ID:** M2-T2
- **Goal:** Enumerate all visible windows on the desktop, recording executable path, window title, and absolute geometry.
- **Files to Create:**
  - `src-tauri/src/window_manager.rs`
- **Files to Modify:**
  - `src-tauri/src/main.rs`
  - `src-tauri/Cargo.toml`
- **Dependencies:** M1-T4
- **Complexity:** Large
- **Risk Level:** High
- **Implementation Notes:** Use `winapi` or `windows-sys` crate on Windows. Use `EnumWindows` callback. Filter out invisible windows, tooling bars, and system overlays using `IsWindowVisible` and title checks. Query process executable path via `QueryFullProcessImageNameW`.
- **Manual Test Steps:** Open Notepad, Paint, and Chrome. Run the enumeration command and verify the output log matches active apps, including titles and dimensions.
- **Acceptance Criteria:** A JSON array of window structures containing `app_name`, `title`, `exe_path`, and coordinates `(x, y, w, h)` is generated accurately.

---

## Milestone 3: The User Experience (Vue 3 + Vuetify 3)

### Task M3-T1: Pause Overlay Interface
- **Task ID:** M3-T1
- **Goal:** Implement the Overlay UI to prompt the user for their note.
- **Files to Create:**
  - `src/views/PauseOverlay.vue`
- **Files to Modify:**
  - `src/App.vue`
  - `src/main.ts`
- **Dependencies:** M1-T1, M1-T4
- **Complexity:** Medium
- **Risk Level:** Low
- **Implementation Notes:** Create a frameless, centered card asking "What should you remember?". Provide a single input text area and a checkbox for "Close everything". Set focus to the input box automatically on mount.
- **Manual Test Steps:** Press `Ctrl+Shift+P`. Verify the overlay appears in the center of the screen, focusing the input field.
- **Acceptance Criteria:** Form mounts, captures user text input, and fires a submit event on hitting Enter.

---

### Task M3-T2: Resume Screen UI
- **Task ID:** M3-T2
- **Goal:** Build the main Resume panel showing the latest captured Moment.
- **Files to Create:**
  - `src/views/ResumeView.vue`
- **Files to Modify:**
  - `src/App.vue`
- **Dependencies:** M1-T1, M3-T1
- **Complexity:** Medium
- **Risk Level:** Low
- **Implementation Notes:** Show the user's words in bold header format. Display the screenshot thumbnail directly beneath the words. Add the "Reopen My Desk" primary button at the bottom.
- **Manual Test Steps:** Open the main app interface. Verify the latest saved Moment text and thumbnail render correctly.
- **Acceptance Criteria:** Words are displayed with correct visual hierarchy; the screenshot thumbnail fits the preview layout.

---

### Task M3-T3: Remember Journal List
- **Task ID:** M3-T3
- **Goal:** Build the historical scroll list of past Moments.
- **Files to Create:**
  - `src/components/RememberList.vue`
- **Files to Modify:**
  - `src/views/ResumeView.vue`
- **Dependencies:** M3-T2
- **Complexity:** Medium
- **Risk Level:** Low
- **Implementation Notes:** Render a simple vertical list of all rows in the database sorted by `created_at` descending. Clicking a row loads it into the main `ResumeView` layout.
- **Manual Test Steps:** Capture multiple moments. Open the main window and confirm the scrollable list displays the timestamps and words of previous moments.
- **Acceptance Criteria:** Historical entries display chronologically; clicking an item updates the primary Resume preview.

---

## Milestone 4: Integration & Window Restoration

### Task M4-T1: Database Integration Service
- **Task ID:** M4-T1
- **Goal:** Connect the frontend UI to the database via Tauri commands (Save Moment, Load Latest, Fetch History, Delete Moment).
- **Files to Create:** None
- **Files to Modify:**
  - `src-tauri/src/main.rs`
  - `src-tauri/src/db.rs`
  - `src/views/ResumeView.vue`
  - `src/views/PauseOverlay.vue`
- **Dependencies:** M1-T2, M2-T1, M2-T2, M3-T1, M3-T2
- **Complexity:** Medium
- **Risk Level:** Medium
- **Implementation Notes:** Expose Tauri commands to write a new Moment record and fetch history. Convert window records to a JSON string prior to insertion. Include screenshot delete logic in the delete command.
- **Manual Test Steps:** Save a new Moment. Confirm it appears in the database and filesystem. Click Delete and verify the database record and file are removed.
- **Acceptance Criteria:** End-to-end data saving, retrieval, and deletion function without errors.

---

### Task M4-T2: Window Restoration Relaunch Service
- **Task ID:** M4-T2
- **Goal:** Implement the restoration mechanism to launch and position applications.
- **Files to Create:** None
- **Files to Modify:**
  - `src-tauri/src/main.rs`
  - `src-tauri/src/window_manager.rs`
- **Dependencies:** M2-T2, M4-T1
- **Complexity:** Large
- **Risk Level:** High
- **Implementation Notes:** Run executable command lines for each application. Wait a short interval (e.g., 500ms) for the window handle to register. Use Win32 `SetWindowPos` to apply recorded geometry (x, y, w, h) based on process ID or matching window title.
- **Manual Test Steps:** Open Notepad and move it to the upper-left. Pause the workspace. Close Notepad. Open August Session, click "Reopen My Desk". Confirm Notepad launches and moves back to the upper-left.
- **Acceptance Criteria:** Executables launch and windows are repositioned to their saved positions.

---

### Task M4-T3: Workspace Window Closure Behavior
- **Task ID:** M4-T3
- **Goal:** Close all captured windows if the user checked "Close everything" during Pause.
- **Files to Create:** None
- **Files to Modify:**
  - `src-tauri/src/window_manager.rs`
- **Files to Modify:**
  - `src-tauri/src/main.rs`
- **Dependencies:** M2-T2, M3-T1
- **Complexity:** Medium
- **Risk Level:** Medium
- **Implementation Notes:** Clean close of all active handles. Send standard close directives.
- **Manual Test Steps:** Confirm active workspace closes when the option is selected.
- **Acceptance Criteria:** Workspace closes gracefully on saving.
