use std::path::Path;

pub fn extract_launch_args(app_name: &str, title: &str) -> Option<String> {
    let app_lower = app_name.to_lowercase();
    
    if app_lower == "code.exe" {
        // VS Code extraction
        if title.ends_with(" - Visual Studio Code") {
            let core = title.trim_end_matches(" - Visual Studio Code");
            let parts: Vec<&str> = core.split(" - ").collect();
            if !parts.is_empty() {
                let folder_name = parts[parts.len() - 1].trim_end_matches(" (Workspace)").trim();
                if !folder_name.is_empty() && folder_name != "Visual Studio Code" {
                    if let Some(dir_path) = find_dir_in_dirs(folder_name) {
                        return Some(dir_path);
                    }
                }
            }
        }
        return None;
    }

    // Identify standard document applications
    let is_doc_app = match app_lower.as_str() {
        "winword.exe" | "excel.exe" | "powerpnt.exe" |
        "acrobat.exe" | "foxitpdfreader.exe" | "sumatrapdf.exe" |
        "notepad.exe" | "notepad++.exe" |
        "photos.exe" | "microsoft.photos.exe" => true,
        _ => false,
    };

    if !is_doc_app {
        return None;
    }

    // Parse filename from window title
    let filename = parse_filename(&app_lower, title)?;
    
    // Check if the filename is already a valid absolute path
    if Path::new(&filename).is_absolute() && Path::new(&filename).exists() {
        return Some(filename);
    }

    // Resolve filename to absolute path by searching common directories
    find_file_in_dirs(&filename)
}

fn parse_filename(app_lower: &str, title: &str) -> Option<String> {
    let mut clean_title = title.trim();

    // Strip leading "*" (indicating unsaved changes in editors)
    if clean_title.starts_with('*') {
        clean_title = clean_title.trim_start_matches('*').trim();
    }

    let filename = match app_lower {
        "winword.exe" => {
            // "Document.docx - Word" or "Document - Word"
            if clean_title.ends_with(" - Word") {
                clean_title.trim_end_matches(" - Word").trim().to_string()
            } else {
                return None;
            }
        }
        "excel.exe" => {
            // "Book.xlsx - Excel" or "Book - Excel"
            if clean_title.ends_with(" - Excel") {
                clean_title.trim_end_matches(" - Excel").trim().to_string()
            } else {
                return None;
            }
        }
        "powerpnt.exe" => {
            // "Presentation.pptx - PowerPoint"
            if clean_title.ends_with(" - PowerPoint") {
                clean_title.trim_end_matches(" - PowerPoint").trim().to_string()
            } else {
                return None;
            }
        }
        "acrobat.exe" => {
            // "Document.pdf - Adobe Acrobat Reader"
            if clean_title.contains(" - Adobe") {
                let parts: Vec<&str> = clean_title.split(" - Adobe").collect();
                parts[0].trim().to_string()
            } else {
                return None;
            }
        }
        "foxitpdfreader.exe" => {
            // "Document.pdf - Foxit PDF Reader"
            if clean_title.ends_with(" - Foxit PDF Reader") {
                clean_title.trim_end_matches(" - Foxit PDF Reader").trim().to_string()
            } else {
                return None;
            }
        }
        "sumatrapdf.exe" => {
            // "Document.pdf - SumatraPDF"
            if clean_title.ends_with(" - SumatraPDF") {
                clean_title.trim_end_matches(" - SumatraPDF").trim().to_string()
            } else {
                return None;
            }
        }
        "notepad.exe" => {
            // "file.txt - Notepad"
            if clean_title.ends_with(" - Notepad") {
                clean_title.trim_end_matches(" - Notepad").trim().to_string()
            } else {
                return None;
            }
        }
        "notepad++.exe" => {
            // "file.txt - Notepad++"
            if clean_title.ends_with(" - Notepad++") {
                clean_title.trim_end_matches(" - Notepad++").trim().to_string()
            } else {
                return None;
            }
        }
        "photos.exe" | "microsoft.photos.exe" => {
            // "image.jpg - Photos"
            if clean_title.ends_with(" - Photos") {
                clean_title.trim_end_matches(" - Photos").trim().to_string()
            } else {
                return None;
            }
        }
        _ => return None,
    };

    if filename.is_empty() {
        return None;
    }

    // Filter out generic unsaved documents (e.g. Document1, Book1, Presentation1, Untitled)
    let is_generic = match app_lower {
        "winword.exe" => filename.starts_with("Document") && filename.chars().skip(8).all(|c| c.is_ascii_digit()),
        "excel.exe" => filename.starts_with("Book") && filename.chars().skip(4).all(|c| c.is_ascii_digit()),
        "powerpnt.exe" => filename.starts_with("Presentation") && filename.chars().skip(12).all(|c| c.is_ascii_digit()),
        "notepad.exe" | "notepad++.exe" => filename.to_lowercase() == "untitled" || filename.to_lowercase() == "new 1",
        _ => false,
    };

    if is_generic {
        None
    } else {
        Some(filename)
    }
}

fn find_file_in_dirs(filename: &str) -> Option<String> {
    let user_profile = std::env::var("USERPROFILE").ok()?;
    let profile_path = Path::new(&user_profile);
    let search_dirs = vec![
        profile_path.join("Desktop"),
        profile_path.join("Documents"),
        profile_path.join("Downloads"),
        profile_path.to_path_buf(),
    ];
    
    // 1. Direct match (depth 1)
    for dir in &search_dirs {
        let path = dir.join(filename);
        if path.is_file() {
            return Some(path.to_string_lossy().into_owned());
        }
    }
    
    // 2. Depth 2 subdirectories
    for dir in &search_dirs {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.filter_map(|e| e.ok()) {
                let path = entry.path();
                if path.is_dir() {
                    let file_path = path.join(filename);
                    if file_path.is_file() {
                        return Some(file_path.to_string_lossy().into_owned());
                    }
                }
            }
        }
    }
    None
}

fn find_dir_in_dirs(dir_name: &str) -> Option<String> {
    let user_profile = std::env::var("USERPROFILE").ok()?;
    let profile_path = Path::new(&user_profile);
    let search_dirs = vec![
        profile_path.join("Desktop"),
        profile_path.join("Documents"),
        profile_path.to_path_buf(),
    ];
    
    // 1. Direct match (depth 1)
    for dir in &search_dirs {
        let path = dir.join(dir_name);
        if path.is_dir() {
            return Some(path.to_string_lossy().into_owned());
        }
    }
    
    // 2. Depth 2 subdirectories
    for dir in &search_dirs {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.filter_map(|e| e.ok()) {
                let path = entry.path();
                if path.is_dir() {
                    let sub_path = path.join(dir_name);
                    if sub_path.is_dir() {
                        return Some(sub_path.to_string_lossy().into_owned());
                    }
                }
            }
        }
    }
    None
}
