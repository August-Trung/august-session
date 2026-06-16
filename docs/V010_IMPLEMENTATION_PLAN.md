# V0.1.0 Implementation Plan: August Session

This document details the software development plan for shipping the initial working version (v0.1.0) of **August Session**—a Workspace Time Machine.

---

## 1. Product Goals
The single overriding goal of v0.1.0 is to deliver the **"Ah, right"** emotional moment.
- **Goal:** Allow the user to capture their immediate mental context (words + screen layout) using a global hotkey, close their active workspace, and restore their mental focus in under 5 seconds when they return.
- **Measure of Success:** A solo user can pause their work, see their own words and screen state later, and immediately resume without cognitive friction.

---

## 2. MVP Scope
The MVP implements the three core verbs exactly as described in the product design:

### PAUSE (The Leaving)
- **Global Hotkey:** A system-wide shortcut (e.g., `Ctrl+Shift+P` on Windows) triggers the capture flow.
- **Screenshot Capture:** Captures the current active desktop display.
- **Window Enumeration:** Captures names, titles, paths, and positions (x, y, w, h) of all active, visible applications.
- **Pause Overlay:** A lightweight, frameless input box that asks: *"What should you remember?"*
- **Persist:** Saves the input, path to screenshot, and window positions as a single `Moment` record.
- **Clear Workspace (Optional):** A checkable option to close all enumerated windows after saving.

### RESUME (The Returning)
- **Main Interface:** Opens automatically when August Session is launched (or via tray menu).
- **Latest Moment Display:** Shows the most recent `Moment`.
  - Displays the user's words in large, clear typography.
  - Displays the desktop screenshot thumbnail below the words.
  - Displays a primary action button: `"Reopen My Desk"`.
- **Reopen & Position:** Reopens the enumerated applications and positions them at their recorded coordinates.

### REMEMBER (The History)
- **Chronological Journal:** A scrollable list of all past `Moments` below the main Resume area.
- **Minimal Metadata:** Each list item shows only the timestamp and the user's words.
- **Historical Restore:** Clicking an item loads it into the main Resume viewer, allowing the user to view the snapshot and trigger `"Reopen My Desk"` for a past moment.

---

## 3. Out-of-Scope (Strictly Excluded)
To ensure shipping velocity and prevent over-engineering, the following are completely excluded:
- **No Background Monitoring/Polling:** The application does not track active windows or analyze activity in the background. It is completely idle until the hotkey is pressed.
- **No Automatic Session Boundaries:** Moments are created only when the user explicitly triggers the Pause hotkey.
- **No Checkpoints/Timeline Scrubbing:** Only discrete, user-initiated Moments are saved.
- **No AI Summarization/Naming:** The name is the user's raw words. No LLM processing.
- **No Deep State Integration:** No browser extensions, browser tab lists, editor file scroll offsets, or cursor positions. Relaunching is limited to the application executable and window bounds.
- **No Cloud Sync/Accounts:** Local-only SQLite database. No network requests.
- **No Custom Window Management/Layouts:** No virtual desktops, tiling manager code, or screen-snapping logic.

---

## 4. Milestones & Timeline
The project is structured into 4 sequential milestones, designed for a solo developer:

```
┌────────────────────────────────────────────────────────┐
│ Milestone 1: Local Data & Shell (Tauri + Rust + SQLite) │
├────────────────────────────────────────────────────────┤
│ Milestone 2: The Capture Engine (Screenshot & Win32)   │
├────────────────────────────────────────────────────────┤
│ Milestone 3: The User Experience (Vue 3 + Vuetify 3)   │
├────────────────────────────────────────────────────────┤
│ Milestone 4: Integration & Window Restoration          │
└────────────────────────────────────────────────────────┘
```

- **Milestone 1: Local Data & Shell (1 week)**
  - Tauri project setup, SQLite initialization, tray icon integration, and database schema setup.
- **Milestone 2: The Capture Engine (1 week)**
  - Rust implementation of global hotkeys, screenshot capture, and native Win32 window enumeration.
- **Milestone 3: The User Experience (1 week)**
  - Frontend implementation of Pause Overlay, Resume screen, and Remember history view using Vue 3 and Vuetify 3.
- **Milestone 4: Integration & Window Restoration (1 week)**
  - Relaunch/reposition service integration, end-to-end flow validation, polish, and binary packaging.

---

## 5. Risks and Mitigations

| Risk | Impact | Mitigation |
|---|---|---|
| Windows API permission issues | High | Verify window enumeration privilege levels early in development. Run Tauri shell at standard user privileges. |
| Application relaunch failures | Medium | Relaunching some apps (like Store apps or UWP apps) by path might fail. Fall back gracefully by showing a warning and skipping the app, rather than crashing the flow. |
| Screenshot capture performance lag | Medium | Capture screen asynchronously immediately upon hotkey press, *before* displaying the Pause Overlay, to ensure the desktop capture matches the exact moment of pausing. |
| Multi-monitor scaling DPI mismatch | Low | Store coordinate sizes in absolute pixels and query DPI scaling via Win32 API to adjust positioning during restoration. |

---

## 6. Acceptance Criteria
1. Pressing `Ctrl+Shift+P` globally opens the Pause overlay in < 500ms.
2. Typing a sentence and hitting Enter saves the Moment and closes the overlay.
3. The main window displays the user's last typed sentence first, followed by a screenshot of the desk.
4. Clicking "Reopen My Desk" launches the recorded applications and repositions their windows back to their original dimensions.
5. Deleting a Moment completely removes its SQLite row and the associated WebP screenshot from the filesystem.
6. CPU usage is 0% and memory footprint is < 50MB when idle in the system tray.

---

## 7. Testing Strategy
- **Manual Verification:** Since the core benefit is visual and spatial, manual validation is the primary driver. Each Milestone defines explicit manual test steps.
- **SQLite Integration Testing:** Basic automated Rust unit tests to verify that CRUD operations on the `Moments` table write, read, and delete correctly.
- **No UI/Frontend Unit Tests:** Rely on Vuetify's structural components and manual interactive validation to keep the development overhead low.

---

## 8. Release Checklist
- [ ] Compile release build: `npm run tauri build`.
- [ ] Verify database initialization on a fresh machine (no pre-existing database file).
- [ ] Validate screenshot directory creation on first start.
- [ ] Test with multiple monitors connected (verify layout doesn't crash or clip).
- [ ] Validate standard application relaunching (e.g., Chrome, Notepad, File Explorer).
- [ ] Confirm no cloud connections or telemetry calls are present.
- [ ] Verify background resource usage after 24 hours of running in tray.
