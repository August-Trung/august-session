# V0.2.0 Code Order: August Session

This document establishes the optimal coding sequence for a solo developer implementing the August Session v0.2.0 Smarter Window Restoration upgrade.

---

## Coding Sequence Strategy

The sequence follows a **data-model-first, incremental-extraction approach**. We first update the core data structure, then implement extraction logic one application type at a time, and finally update the restoration engine.

```
       ┌────────────────────────────────────────────────────────┐
       │ 1. Extend WindowInfo struct (Data model baseline)      │
       └───────────────────────────┬────────────────────────────┘
                                   ▼
       ┌────────────────────────────────────────────────────────┐
       │ 2. File Explorer COM extraction (Highest complexity)   │
       └───────────────────────────┬────────────────────────────┘
                                   ▼
       ┌────────────────────────────────────────────────────────┐
       │ 3. Title parser for Office/PDF/Text (Most user value)  │
       └───────────────────────────┬────────────────────────────┘
                                   ▼
       ┌────────────────────────────────────────────────────────┐
       │ 4. VS Code + Browser flags (Quick wins)                │
       └───────────────────────────┬────────────────────────────┘
                                   ▼
       ┌────────────────────────────────────────────────────────┐
       │ 5. Smart restore engine + dedup + polish               │
       └────────────────────────────────────────────────────────┘
```

---

## Detailed Development Stages

### Stage 1: Data Model Update
1. **Extend WindowInfo** ([M5-T1](file:///d:/Study/Projects/000-Desktop_App/August-Session/docs/V020_IMPLEMENTATION_TASKS.md#task-m5-t1-extend-windowinfo-with-launch_args-field)): Adds the `launch_args` field. All subsequent extraction logic writes to this field.

*Checkpoint 1:* The app compiles and runs. Existing window enumeration works as before. The `launch_args` field appears as `null` in JSON output. Old Moments still deserialize correctly.

---

### Stage 2: File Explorer Extraction (High Risk)
1. **File Explorer COM API** ([M5-T2](file:///d:/Study/Projects/000-Desktop_App/August-Session/docs/V020_IMPLEMENTATION_TASKS.md#task-m5-t2-file-explorer-folder-path-extraction-com-api)): The most technically challenging part of v0.2.0. Uses Windows COM interfaces to extract folder paths from File Explorer windows.

*Checkpoint 2:* Opening 3 File Explorer folders and running enumeration shows the correct folder paths in `launch_args` for each Explorer window.

---

### Stage 3: Document Title Parsing (High User Value)
1. **Title Parser Module** ([M5-T3](file:///d:/Study/Projects/000-Desktop_App/August-Session/docs/V020_IMPLEMENTATION_TASKS.md#task-m5-t3-document-file-path-extraction-title-parsing)): Implements title parsing and filesystem search for Office documents, PDFs, text files, and images.
2. **VS Code Extension** ([M5-T4](file:///d:/Study/Projects/000-Desktop_App/August-Session/docs/V020_IMPLEMENTATION_TASKS.md#task-m5-t4-vs-code-workspace-path-extraction)): Adds VS Code case to the title parser.

*Checkpoint 3:* Opening a Word document, a PDF, and a VS Code project shows the correct file/folder paths in `launch_args`.

---

### Stage 4: Browser Flags & Restoration
1. **Browser Session Flags** ([M5-T5](file:///d:/Study/Projects/000-Desktop_App/August-Session/docs/V020_IMPLEMENTATION_TASKS.md#task-m5-t5-browser-session-flag-assignment)): Assigns `--restore-last-session` to browser windows.
2. **Smart Restore Engine** ([M6-T1](file:///d:/Study/Projects/000-Desktop_App/August-Session/docs/V020_IMPLEMENTATION_TASKS.md#task-m6-t1-update-restore_windows-to-use-launch_args)): Updates `ShellExecuteW` to pass `launch_args` as parameters.
3. **Browser Deduplication** ([M6-T2](file:///d:/Study/Projects/000-Desktop_App/August-Session/docs/V020_IMPLEMENTATION_TASKS.md#task-m6-t2-browser-deduplication-logic)): Prevents multiple browser instances from launching.

*Checkpoint 4:* Full Pause → Restore cycle. File Explorer opens to the right folders. Word opens the right document. Browser restores its session. All in one click.

---

### Stage 5: Testing & Polish
1. **Backward Compatibility** ([M7-T1](file:///d:/Study/Projects/000-Desktop_App/August-Session/docs/V020_IMPLEMENTATION_TASKS.md#task-m7-t1-backward-compatibility-verification)): Verify old Moments still work.
2. **Cross-Application Testing** ([M7-T2](file:///d:/Study/Projects/000-Desktop_App/August-Session/docs/V020_IMPLEMENTATION_TASKS.md#task-m7-t2-end-to-end-cross-application-testing)): Comprehensive manual test scenario.
3. **Error Handling** ([M7-T3](file:///d:/Study/Projects/000-Desktop_App/August-Session/docs/V020_IMPLEMENTATION_TASKS.md#task-m7-t3-error-handling--graceful-fallback)): Ensure no crashes under any failure scenario.

*Checkpoint 5:* All tests pass. Old and new Moments restore correctly. No crashes on edge cases.

---

## Why This Sequence Minimizes Risk
1. **Data Model First:** Extending `WindowInfo` before writing extraction logic ensures the contract is clear and backward compatibility is verified immediately.
2. **High-Risk-First:** The COM API for File Explorer is the most complex OS integration. Building it early exposes problems before we invest time in simpler extractors.
3. **User Value Next:** Office/PDF/text title parsing covers the broadest set of everyday users (teachers, students, office workers) and is implemented right after the hardest piece.
4. **Quick Wins Last:** Browser flags and VS Code are straightforward and can be done quickly once the extraction pattern is established.
5. **Incremental Testing:** Each stage produces a runnable application that can be manually verified before moving on.
