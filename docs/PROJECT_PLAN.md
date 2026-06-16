# Project Plan

*Each phase must earn the right to exist by proving the previous one matters.*

---

## Prototype 0 — The Emotional Core

**Goal:** Prove that your-words-plus-your-desk is enough to make someone say "ah, right."

**Build:**

- Electron app that lives in the system tray
- Global hotkey registers a Pause
- On Pause:
  - Capture a desktop screenshot (Electron desktopCapturer)
  - Enumerate open windows (native Win32 APIs)
  - Show a small overlay: "What should you remember?"
  - Save the Moment (words, screenshot, windows, timestamp) to SQLite + filesystem
- On open:
  - Show the most recent Moment: words first, screenshot below
  - "Reopen My Desk" button: launches executables and positions windows
  - Scroll down to see past Moments (timestamp + words)

**Does not include:**

- Polish
- Settings
- Window close behavior
- Error handling beyond basics
- Installers

**Success means:** Using it yourself for a week and finding that the morning "ah, right" moment consistently happens. Not the window reopening. The *recognition*.

**Estimated effort:** 2–3 weeks for one developer.

---

## Prototype 1 — Better Window Restoration

**Earns the right to exist if:** Prototype 0 proves the emotional core works, and you find that window reopening is useful but the positioning is unreliable.

**Build:**

- Improve window positioning reliability
  - Handle multi-monitor setups
  - Handle DPI scaling differences
  - Wait for application windows to appear before positioning
  - Retry positioning if the window isn't ready
- Handle edge cases
  - Applications that open multiple windows
  - Applications that restore their own window positions
  - Applications that no longer exist on the system
- Add a "close everything" option to the Pause overlay
- Basic error handling: if an app fails to launch, skip it gracefully
- Minimal settings: change the global hotkey, set launch-on-login

**Does not include:**

- Browser tab restoration
- Any application-specific deep state
- Any automatic behavior

**Success means:** Window reopening works reliably enough that you trust it. You press "Reopen My Desk" and things appear roughly where they should, without errors or surprises.

**Estimated effort:** 2–3 weeks.

---

## Prototype 2 — Browser Tab Restoration

**Earns the right to exist if:** You find yourself using August daily, and the most common frustration is that the browser reopens but your tabs are gone.

**Build:**

- A simple browser extension (Chrome, then Firefox) that:
  - On request from August: reports the list of open tabs (URL + title)
  - On request from August: opens a list of URLs as tabs
- August captures browser tabs as part of the Moment's window data
- "Reopen My Desk" restores browser windows with their tabs
- The extension is optional — August works without it, just without tab restoration

**Does not include:**

- Scroll positions within pages
- Browser history
- Tab groups or ordering
- Any other application-specific integration

**Success means:** When you resume, the browser opens with the right tabs. The research you were doing is right there. The pages you were comparing are side by side again.

**Estimated effort:** 2–3 weeks (including extension review/publishing).

---

## Prototype 3 — Deep State (Selective)

**Earns the right to exist if:** Browser tab restoration proves valuable, and you discover specific, recurring frustrations — like a document opening to page 1 instead of where you were reading, or a video restarting from the beginning.

**Build only what real pain demands.** Possible additions, each independent:

- **Scroll position in browser tabs** — extend the browser extension to record and restore scroll position for the active tab
- **Document position** — for specific, common document viewers, record which page was being viewed
- **Media playback position** — for common media players, record the timestamp

Each of these is a small, self-contained addition. Not a framework. Not a plugin system. A specific solution to a specific pain.

**Does not include:**

- A general-purpose "deep state" framework
- A plugin architecture
- Application-specific integrations beyond what's listed
- Anything speculative

**Success means:** The specific frustration that motivated the work is gone. Nothing more.

**Estimated effort:** Variable. Each addition is 1–2 weeks.

---

## What Is Not On The Roadmap

The following are not planned, not designed, and not promised. They do not have "Prototype N" labels waiting for them.

- Cloud sync
- Mobile app
- Team features
- AI summarization
- Activity tracking
- Productivity analytics
- Plugin ecosystem
- Marketplace
- Social features
- API

If any of these ever become necessary, it will be because daily use revealed a genuine, persistent need — not because they were on a roadmap.

---

## Pace

There is no deadline. Each prototype is used daily before the next one begins. The cadence is:

1. Build the prototype
2. Use it every day
3. Notice what's missing
4. Decide if what's missing is real or imagined
5. If real, build the next prototype
6. If imagined, stay where you are

The product advances by subtraction of frustration, not by addition of features.
