# V0.2.0 Implementation Tasks: August Session

This document details the granular, step-by-step programming tasks required to implement the v0.2.0 Smarter Window Restoration upgrade of August Session.

---

## Milestone 5: Launch Argument Extraction

### Task M5-T1: Extend WindowInfo with launch_args Field
- **Task ID:** M5-T1
- **Goal:** Add an optional `launch_args` field to the `WindowInfo` struct to store the file path, folder path, or launch flag associated with each window.
- **Files to Create:** None
- **Files to Modify:**
  - `src-tauri/src/window_manager.rs`
- **Dependencies:** None (builds on existing v0.1.0 code)
- **Complexity:** Small
- **Risk Level:** Low
- **Implementation Notes:** Add `pub launch_args: Option<String>` to `WindowInfo`. Since the field uses `Option<String>` and `serde`, old Moments without this field will deserialize to `None` automatically, preserving backward compatibility. Update the `enum_windows_with_handles_callback` to initialize `launch_args` to `None` by default.
- **Manual Test Steps:** Compile and run the app. Verify existing window enumeration still works and the new field appears as `null` in the JSON output.
- **Acceptance Criteria:** `WindowInfo` includes `launch_args` field. Old Moments still deserialize without errors.

---

### Task M5-T2: File Explorer Folder Path Extraction (COM API)
- **Task ID:** M5-T2
- **Goal:** Extract the current folder path from open File Explorer windows using Windows COM automation (`IShellWindows`).
- **Files to Create:**
  - `src-tauri/src/explorer.rs`
- **Files to Modify:**
  - `src-tauri/src/main.rs` (add `mod explorer;`)
  - `src-tauri/src/window_manager.rs` (call explorer extraction)
  - `src-tauri/Cargo.toml` (add `windows` crate dependency)
- **Dependencies:** M5-T1
- **Complexity:** Large
- **Risk Level:** High
- **Implementation Notes:** Use the `windows` crate to access `IShellWindows` COM interface. For each visible File Explorer window, query the `IWebBrowserApp` interface to retrieve the `LocationURL`. Convert the URL format (`file:///C:/path`) to a standard Windows path (`C:\path`). Match Explorer windows by comparing the window handle (HWND) obtained through `IShellWindows` with the enumerated HWND. Store the folder path in `launch_args`.
- **Manual Test Steps:** Open 3 File Explorer windows to different folders (Desktop, Documents, Downloads). Run the enumeration. Verify each Explorer window's `launch_args` contains the correct absolute folder path.
- **Acceptance Criteria:** File Explorer windows have their `launch_args` populated with the correct folder path. Non-Explorer windows are unaffected.

---

### Task M5-T3: Document File Path Extraction (Title Parsing)
- **Task ID:** M5-T3
- **Goal:** Extract the file path of documents open in Microsoft Office (Word, Excel, PowerPoint), PDF readers, Notepad, and image viewers by parsing the window title and searching the filesystem.
- **Files to Create:**
  - `src-tauri/src/title_parser.rs`
- **Files to Modify:**
  - `src-tauri/src/main.rs` (add `mod title_parser;`)
  - `src-tauri/src/window_manager.rs` (call title parser)
- **Dependencies:** M5-T1
- **Complexity:** Medium
- **Risk Level:** Medium
- **Implementation Notes:** Implement `extract_file_from_title(app_name, title) -> Option<String>` which:
  1. Identifies the application type from the `app_name` (e.g., `WINWORD.EXE`, `EXCEL.EXE`, `POWERPNT.EXE`, `Acrobat.exe`, `FoxitPDFReader.exe`, `SumatraPDF.exe`, `notepad.exe`, `notepad++.exe`, `Photos.exe`, `Microsoft.Photos.exe`).
  2. Parses the window title to extract the filename. Common title formats:
     - Word: `"Document Name - Word"` or `"Document Name.docx - Word"`
     - Excel: `"Workbook Name - Excel"` or `"Workbook Name.xlsx - Excel"`
     - PowerPoint: `"Presentation Name - PowerPoint"`
     - Adobe Reader: `"Document.pdf - Adobe Acrobat Reader"`
     - Notepad: `"filename.txt - Notepad"` or `"*filename.txt - Notepad"` (unsaved changes)
     - Notepad++: `"*filename.ext - Notepad++"` or `"filename.ext - Notepad++"`
  3. Searches for the file in common user directories: `Desktop`, `Documents`, `Downloads`, and the user's home directory.
  4. Returns the first match as an absolute path, or `None` if not found.
- **Manual Test Steps:** Open a Word document from Documents folder. Open a PDF from Downloads. Open a text file from Desktop. Run enumeration and verify each window's `launch_args` contains the correct file path.
- **Acceptance Criteria:** Windows for supported applications have `launch_args` populated with the correct file path when the file is found. Unsupported or unfound files return `None`.

---

### Task M5-T4: VS Code Workspace Path Extraction
- **Task ID:** M5-T4
- **Goal:** Extract the project folder path from VS Code windows by parsing the window title.
- **Files to Create:** None
- **Files to Modify:**
  - `src-tauri/src/title_parser.rs` (add VS Code case)
- **Dependencies:** M5-T1, M5-T3
- **Complexity:** Small
- **Risk Level:** Low
- **Implementation Notes:** VS Code window titles follow the pattern: `"[File] - [FolderName] - Visual Studio Code"` or `"[FolderName] - Visual Studio Code"`. Extract the folder name. To resolve the absolute path, search for a directory matching the folder name inside common workspace locations (Desktop, Documents, user home, common dev directories). Alternatively, check VS Code's recently opened workspaces from its settings database at `%APPDATA%\Code\User\globalStorage\state.vscdb`.
- **Manual Test Steps:** Open VS Code to a project folder. Run enumeration and verify `launch_args` contains the correct absolute project path.
- **Acceptance Criteria:** VS Code windows have `launch_args` populated with the correct project folder path.

---

### Task M5-T5: Browser Session Flag Assignment
- **Task ID:** M5-T5
- **Goal:** Assign the `--restore-last-session` flag as `launch_args` for browser windows.
- **Files to Create:** None
- **Files to Modify:**
  - `src-tauri/src/window_manager.rs` (add browser detection in enumeration)
- **Dependencies:** M5-T1
- **Complexity:** Small
- **Risk Level:** Low
- **Implementation Notes:** During enumeration, check if `app_name` matches known browser executables: `msedge.exe`, `chrome.exe`, `firefox.exe`, `brave.exe`, `opera.exe`. If matched, set `launch_args` to `"--restore-last-session"`. Mark all browser windows from the same `exe_path` with a flag to prevent duplicate launches during restoration (only the first window per browser should trigger the session restore).
- **Manual Test Steps:** Open Edge with multiple tabs. Run enumeration. Verify the browser window has `launch_args` set to `"--restore-last-session"`.
- **Acceptance Criteria:** Browser windows have `launch_args` populated. Multiple windows from the same browser do not result in duplicate launches.

---

## Milestone 6: Smart Restoration Engine

### Task M6-T1: Update restore_windows to Use launch_args
- **Task ID:** M6-T1
- **Goal:** Modify the `restore_windows` function to pass `launch_args` as the argument parameter to `ShellExecuteW` when available.
- **Files to Create:** None
- **Files to Modify:**
  - `src-tauri/src/window_manager.rs`
- **Dependencies:** M5-T1, M5-T2, M5-T3, M5-T4, M5-T5
- **Complexity:** Medium
- **Risk Level:** Medium
- **Implementation Notes:** Update the `ShellExecuteW` call:
  - If `launch_args` is `Some(args)`, convert the args to a wide string and pass it as the `lpParameters` parameter.
  - If `launch_args` is `None`, pass `std::ptr::null()` as before (v0.1.0 fallback behavior).
  - For File Explorer, use the folder path directly as the parameter to `explorer.exe`.
  - For browsers, use the `--restore-last-session` flag. Deduplicate: only launch one instance per browser executable, skipping duplicate windows from the same `exe_path`.
  - For documents, pass the absolute file path as the parameter.
- **Manual Test Steps:** Save a Moment with Explorer folders, a Word doc, a PDF, and a browser. Click "Reopen My Desk". Verify each application opens with its content.
- **Acceptance Criteria:** Applications open with the correct content. Applications without `launch_args` fall back to v0.1.0 behavior.

---

### Task M6-T2: Browser Deduplication Logic
- **Task ID:** M6-T2
- **Goal:** Ensure that multiple browser windows captured during a single Moment do not cause multiple browser instances to launch during restoration.
- **Files to Create:** None
- **Files to Modify:**
  - `src-tauri/src/window_manager.rs`
- **Dependencies:** M6-T1
- **Complexity:** Small
- **Risk Level:** Low
- **Implementation Notes:** During `restore_windows`, maintain a `HashSet<String>` of already-launched `exe_path` values. Before launching a browser (identified by `app_name` matching known browser names), check if its `exe_path` has already been launched. If yes, skip it. The first browser window triggers `--restore-last-session`, which restores all tabs and windows from the previous session.
- **Manual Test Steps:** Open Edge with 3 separate windows. Pause and restore. Verify only one Edge instance launches (with session restore bringing back all windows).
- **Acceptance Criteria:** Only one browser process is launched per browser type during restoration.

---

## Milestone 7: Integration Testing & Polish

### Task M7-T1: Backward Compatibility Verification
- **Task ID:** M7-T1
- **Goal:** Verify that existing v0.1.0 Moments (without `launch_args`) still restore correctly.
- **Files to Create:** None
- **Files to Modify:** None
- **Dependencies:** M6-T1
- **Complexity:** Small
- **Risk Level:** Low
- **Implementation Notes:** Load old Moments from the database. The JSON deserialization of `WindowInfo` should treat the missing `launch_args` field as `None` thanks to `#[serde(default)]`. Verify the restoration logic falls back to v0.1.0 behavior (launching exe without arguments).
- **Manual Test Steps:** Without clearing the database, click "Reopen My Desk" on an old v0.1.0 Moment. Verify applications launch without errors.
- **Acceptance Criteria:** Old Moments restore with v0.1.0 behavior. No deserialization errors.

---

### Task M7-T2: End-to-End Cross-Application Testing
- **Task ID:** M7-T2
- **Goal:** Perform comprehensive manual testing across all supported application types.
- **Files to Create:** None
- **Files to Modify:** None
- **Dependencies:** M6-T1, M6-T2, M7-T1
- **Complexity:** Medium
- **Risk Level:** Low
- **Implementation Notes:** Open a representative mix of applications:
  1. File Explorer → 2 different folders
  2. Microsoft Word → a `.docx` file from Documents
  3. Notepad → a `.txt` file from Desktop
  4. Edge → 5+ tabs open
  5. (Optional) VS Code → a project folder
  6. (Optional) A PDF reader → a `.pdf` file
  
  Press `Ctrl+Shift+P`, type a note, check "Close everything", press Done. Open August Session, verify the Moment is saved. Click "Reopen My Desk" and verify all applications open with their content.
- **Manual Test Steps:** Follow the scenario above. Document results for each application type.
- **Acceptance Criteria:** All supported applications reopen with their content. Unsupported applications open normally. No crashes or errors.

---

### Task M7-T3: Error Handling & Graceful Fallback
- **Task ID:** M7-T3
- **Goal:** Ensure that extraction or restoration failures degrade gracefully to v0.1.0 behavior instead of crashing.
- **Files to Create:** None
- **Files to Modify:**
  - `src-tauri/src/window_manager.rs`
  - `src-tauri/src/explorer.rs`
  - `src-tauri/src/title_parser.rs`
- **Dependencies:** M6-T1
- **Complexity:** Small
- **Risk Level:** Low
- **Implementation Notes:** Wrap all extraction functions in error handling:
  - If COM API fails for File Explorer, set `launch_args = None` and continue.
  - If title parsing fails or file is not found, set `launch_args = None` and continue.
  - If `ShellExecuteW` fails with arguments, retry without arguments as fallback.
  - Log warnings for debugging but never crash the application.
- **Manual Test Steps:** Rename a previously saved document file. Try restoring the Moment. Verify the application opens without the file (graceful fallback) instead of crashing.
- **Acceptance Criteria:** No crashes under any failure scenario. Failed extractions silently fall back to v0.1.0 behavior.
