# V0.2.0 Implementation Plan: August Session

This document details the software development plan for shipping **v0.2.0** of **August Session**—Smarter Window Restoration.

---

## 1. Product Goals
The single overriding goal of v0.2.0 is to deliver a **"It remembered everything"** emotional moment.
- **Goal:** When the user clicks "Reopen My Desk", applications reopen with their **content** — the folder they were browsing, the document they were editing, the pages they were reading — not just an empty application window.
- **Measure of Success:** A solo user can pause their work, and when they resume, the applications reopen to the same files, folders, and content they were working with. No manual re-navigation needed.

---

## 2. Upgrade Scope
v0.2.0 builds on the existing v0.1.0 foundation. No architectural changes are needed. The upgrade adds a single concept: **launch arguments** — the file path, folder path, or URL that tells each application *what to open*, not just *which program to run*.

### What Changes

| Application Type | v0.1.0 Behavior | v0.2.0 Behavior |
|---|---|---|
| **File Explorer** | Opens to default location | Opens to the exact folder the user was browsing |
| **Microsoft Word** | Opens blank document | Opens the exact `.docx` file being edited |
| **Microsoft Excel** | Opens blank spreadsheet | Opens the exact `.xlsx` file being edited |
| **Microsoft PowerPoint** | Opens blank presentation | Opens the exact `.pptx` file being edited |
| **Adobe Reader / Foxit / SumatraPDF** | Opens empty reader | Opens the exact PDF being read |
| **Notepad / Notepad++** | Opens empty editor | Opens the exact text file being edited |
| **Web Browsers (Edge/Chrome)** | Opens to homepage | Restores with `--restore-last-session` flag |
| **VS Code** | Opens welcome screen | Opens the exact project folder / workspace |
| **Image Viewers** | Opens empty viewer | Opens the exact image being viewed |
| **Other applications** | Opens normally (unchanged) | Opens normally (unchanged — graceful fallback) |

### What Does NOT Change
- Database schema stays the same (new `launch_args` field is `Option<String>`, backward-compatible with existing Moments)
- Frontend UI stays the same (no new screens or buttons)
- Screenshot capture stays the same
- Window enumeration stays the same (enhanced with argument extraction)
- Tray menu and hotkey stay the same

---

## 3. Out-of-Scope (Strictly Excluded)
- **No Browser Extension:** Tab restoration uses the browser's built-in `--restore-last-session` flag. No extension needed.
- **No Scroll Position Capture:** Documents open to page 1, not the exact scroll offset. That's Prototype 3 territory.
- **No Tab-Level Granularity:** Browser tabs are restored as a group using the browser's session restore, not individually by URL.
- **No UWP / Windows Store Apps:** Store apps (Calculator, Settings, etc.) remain filtered out.
- **No Background Monitoring:** Still completely idle until the hotkey is pressed.

---

## 4. Milestones & Timeline
The upgrade is structured into 3 sequential milestones:

```
┌─────────────────────────────────────────────────────────────────┐
│ Milestone 5: Launch Argument Extraction (WindowInfo upgrade)    │
├─────────────────────────────────────────────────────────────────┤
│ Milestone 6: Smart Restoration Engine (launch with arguments)   │
├─────────────────────────────────────────────────────────────────┤
│ Milestone 7: Integration Testing & Polish                       │
└─────────────────────────────────────────────────────────────────┘
```

- **Milestone 5: Launch Argument Extraction (1 week)**
  - Extend `WindowInfo` with `launch_args`. Implement argument extraction for File Explorer (COM API), Office/PDF/text editors (title parsing + file search), browsers (session flag), and VS Code (title parsing).
- **Milestone 6: Smart Restoration Engine (3–4 days)**
  - Update `restore_windows` to pass `launch_args` to `ShellExecuteW`. Handle deduplication for browsers.
- **Milestone 7: Integration Testing & Polish (2–3 days)**
  - End-to-end testing across all supported application types. Verify backward compatibility with v0.1.0 Moments.

---

## 5. Risks and Mitigations

| Risk | Impact | Mitigation |
|---|---|---|
| File Explorer COM API complexity | High | COM API (`IShellWindows`) is well-documented. Use the `windows` crate for safe bindings. Fall back to title-based folder search if COM fails. |
| Office files may not have full path in title | Medium | Search common directories (Documents, Desktop, Downloads, recent files) for the filename from the title. Fall back to opening the application without arguments if not found. |
| Browser `--restore-last-session` may conflict with existing sessions | Low | Only pass the flag if the browser was captured during the Pause. Document this behavior for users. |
| Backward compatibility with v0.1.0 data | Low | `launch_args` is `Option<String>`. Old Moments without this field deserialize to `None`, and restoration falls back to v0.1.0 behavior. |

---

## 6. Acceptance Criteria
1. Pausing while 3 different File Explorer folders are open, then clicking "Reopen My Desk" reopens all 3 folders at their exact directories.
2. Pausing while editing a Word document, then restoring, opens the exact `.docx` file.
3. Pausing while browsing in Edge with multiple tabs, then restoring, reopens Edge with session restore.
4. Pausing with VS Code open to a project, then restoring, reopens VS Code to the same project folder.
5. Old v0.1.0 Moments (without `launch_args`) still restore correctly using the fallback behavior.
6. Applications that cannot be matched to arguments still open normally (graceful degradation).

---

## 7. Testing Strategy
- **Manual Verification:** Primary testing method. Each Milestone defines explicit manual test steps covering the target applications.
- **Backward Compatibility Check:** Load existing v0.1.0 Moments and verify they restore without errors.
- **No UI/Frontend Unit Tests:** No frontend changes are made in v0.2.0.
