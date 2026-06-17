# August Session — Workspace Time Machine

> *A letter from who you are now to who you'll be tomorrow.*

**August Session** is a minimalist, secure desktop application designed to preserve and restore the continuity of your attention. Whenever you need to pause your work (for a meeting, dinner, sleep, or the weekend), August Session captures your current mental state and your open windows, allowing you to return exactly to where you left off.

---

## 🌟 The Insight
August Session does not just restore your windows; it restores your **mind**. The mind is recovered through recognition, not reconstruction. 

By writing a single context-rich sentence when you pause, and looking at a "photograph" of your desk tomorrow, your brain immediately recalls the thread of your attention.

---

## 🚀 Key Features

### v0.1.0 — The Emotional Core
- **Global Hotkey (`Ctrl+Shift+P`)**: Triggers a Pause overlay from anywhere instantly.
- **Visual Desktop Capture**: Snaps a screenshot of your workspace as you leave.
- **Context Note**: Write one quick sentence of what you were about to try next.
- **Graceful Dismissal**: Double-check options like "Close everything" or cancel anytime.

### v0.2.0 — Smarter Window Restoration
- **File Explorer Restoration**: Reopens File Explorer directly to the folders you were browsing.
- **Document Restoration**: Parses titles and resolves filenames for **Microsoft Office** (Word, Excel, PowerPoint), **PDF Readers** (Adobe Acrobat, Foxit, SumatraPDF), **Notepad/Notepad++**, and **Photos**, looking them up in `Desktop`, `Documents`, or `Downloads`.
- **DPI & Multi-Monitor Aware Position Mapping**: Windows are restored exactly where they were, mapped one-to-one to avoid overlap.

### v0.3.0 — Companion Browser Extension & Granular Tab Restoration
- **Granular Tab Capture**: Uses a lightweight browser extension to capture the exact list of URLs open in Chrome and Edge.
- **Multi-Window Browser Restoration**: Reopens browsers with the exact tab lists in separate windows, grouped correctly.
- **Local HTTP Server Integration**: Built-in background server in Rust (`localhost:18942`) communicate safely using CORS protection.
- **Optional & Secure**: Runs entirely offline on your machine. If the extension is not installed, it falls back to the browser's built-in session restore.

---

## 🛠️ Tech Stack
- **Backend**: Rust, Tauri v1, SQLite (via `rusqlite`), Win32 COM Automation, `tiny_http`
- **Frontend**: Vue 3, TypeScript, Vite, Vuetify 3 (Material Design)

---

## 📦 Getting Started

### Prerequisites
- [Rust & Cargo](https://www.rust-lang.org/tools/install)
- [Node.js & npm](https://nodejs.org/)

### Run Development App
1. Clone the repository:
   ```bash
   git clone https://github.com/August-Trung/august-session.git
   cd august-session
   ```
2. Install frontend dependencies:
   ```bash
   npm install
   ```
3. Run the application in development mode:
   ```bash
   npm run tauri dev
   ```

### Install Companion Browser Extension (Chrome/Edge)
1. Open Chrome/Edge and navigate to `chrome://extensions/` or `edge://extensions/`.
2. Toggle **Developer mode** on (top right).
3. Click **Load unpacked** and select the `browser-extension` folder in the project root directory.

---

## 📝 License
Built for personal productivity and continuity of attention. Open source.
