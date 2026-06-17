# V0.3.0 Implementation Plan: August Session

This document details the software design and development plan for shipping **v0.3.0** of **August Session** — Companion Browser Extension & Granular Tab Restoration.

---

## 1. Product Goals
The primary goal of v0.3.0 is to make browser restoration **100% reliable and granular**.
- **Goal:** Instead of relying on the browser's built-in session restore (which can be volatile, override sessions, and fail when the browser is already running), August Session will capture the **exact list of URLs** open in all browser windows and reopen them precisely.
- **Success Criteria:** Pausing while having multiple browser windows with specific tabs open, and resuming, restores the exact browser windows with the correct tabs in the correct order, even if the browser was already running.

---

## 2. Upgrade Scope
v0.3.0 introduces a browser extension and a communication API in the Rust backend.

### What Changes

| Component | v0.2.0 Behavior | v0.3.0 Behavior |
|---|---|---|
| **Edge / Chrome Restoration** | Reopens browser with `--restore-last-session` | Reopens browser with the exact URLs captured (e.g., `msedge.exe "url1" "url2"`) |
| **Tauri Backend** | Static window enumeration only | Launches a lightweight background thread running a local HTTP server (`localhost:18942`) |
| **Browser Extension** | None (Not used) | A lightweight companion extension captures tabs (URLs + Titles) on request |
| **WindowInfo Database Schema** | `launch_args` stores `"--restore-last-session"` | `launch_args` stores JSON-encoded list of URLs for browser windows |
| **Multi-window browser support** | Only launches one browser instance | Launches each captured browser window with its specific set of tabs |

### What Does NOT Change
- Restoration logic for File Explorer, Microsoft Office, PDF readers, Notepad, and VS Code (remains unchanged from v0.2.0).
- Frontend UI screens (remains clean and simple).

---

## 3. Communication Architecture
The Tauri desktop app and the Browser Extension communicate locally:

```
┌───────────────────────────┐                  ┌───────────────────────────┐
│                           │  Get Tabs JSON   │                           │
│    Tauri Desktop App      ├─────────────────►│     Browser Extension     │
│   (Local HTTP Client)     │◄─────────────────┤    (Background Script)    │
│                           │   Tab List JSON  │                           │
└───────────────────────────┘                  └───────────────────────────┘
```

### Protocol: Local HTTP Server in Extension
Because native extensions cannot run standard HTTP servers, the communication is designed as follows:
1. **Lightweight Local Server in Rust**: Tauri starts a background HTTP server (using `tiny_http` or a basic `TcpListener` to keep dependencies minimal) listening on `localhost:18942`.
2. **Long-poll / WebSocket Connection**: The extension connects to Tauri's local server.
3. **Trigger Capture**:
   - When the user presses `Ctrl+Shift+P`, Tauri sends a query request to `localhost:18942` (which the extension reads).
   - The extension queries the browser tabs using the `chrome.tabs` API.
   - The extension sends the tabs data (window ID, tab index, URL, title) back as an HTTP POST request.
   - Tauri saves the URLs in the Moment's database window layout.

---

## 4. Milestones & Timeline

```
┌─────────────────────────────────────────────────────────────────┐
│ Milestone 8: Companion Browser Extension (Chrome/Edge manifest) │
├─────────────────────────────────────────────────────────────────┤
│ Milestone 9: Tauri Local HTTP API & Capture Integration         │
├─────────────────────────────────────────────────────────────────┤
│ Milestone 10: Granular Tab Restoration & Multi-window Support   │
└─────────────────────────────────────────────────────────────────┘
```

### Milestone 8: Companion Browser Extension
- Create extension directory `browser-extension/`.
- Implement `manifest.json` (Manifest V3) supporting Chrome and Edge.
- Write `background.js` to query browser windows and tabs using `chrome.tabs.query`.
- Implement reconnect loop to communicate with Tauri's local server.

### Milestone 9: Tauri Local Server API
- Implement a lightweight, zero-dependency TCP loop in `src-tauri/src/api_server.rs` or add `tiny_http`.
- Integrate server startup in `main.rs` setup.
- Update `save_moment` command to request tabs from the server, waiting for a maximum of 250ms timeout.
- Map browser windows to the captured URLs list.

### Milestone 10: Granular Tab Restoration
- Update `restore_windows` in `window_manager.rs` to parse the URLs list.
- Launch browser windows with specific URL arguments.
- Handle active/existing browser instances gracefully.

---

## 5. Risks and Mitigations

| Risk | Mitigation |
|---|---|
| Extension is optional (user might not install it) | Fall back gracefully to v0.2.0 behavior (`--restore-last-session` or standard launcher) if the extension is not connected. |
| Security: Malicious sites accessing local server | Add origin check: only allow requests originating from `chrome-extension://` and verify a unique session token generated by Tauri on startup. |
| Port 18942 already in use | Try a range of ports (e.g. 18942 - 18945) and notify the extension of the port in use. |

---

## 6. Acceptance Criteria
1. Pausing with Chrome having 3 tabs open, and Edge having 2 tabs open: restoring reopens Chrome to those 3 tabs, and Edge to those 2 tabs.
2. If the user does not have the extension installed, browser restoration falls back to opening the browser to the default screen or using the `--restore-last-session` flag.
3. Secure communication: arbitrary web pages cannot fetch tab data or command Tauri to launch applications.
