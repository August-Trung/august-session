use serde::{Deserialize, Serialize};
use std::ffi::OsString;
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::path::Path;
use winapi::shared::minwindef::{BOOL, DWORD, LPARAM};
use winapi::shared::windef::{HWND, RECT};
use winapi::um::handleapi::CloseHandle;
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::winbase::QueryFullProcessImageNameW;
use winapi::um::winuser::{
    EnumWindows, GetAncestor, GetWindowRect, GetWindowTextLengthW, GetWindowTextW,
    GetWindowThreadProcessId, IsWindowVisible, GA_ROOTOWNER,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WindowInfo {
    pub app_name: String,
    pub title: String,
    pub exe_path: String,
    #[serde(default)]
    pub launch_args: Option<String>,
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

pub struct WindowWithHandle {
    pub info: WindowInfo,
    pub hwnd: HWND,
}

fn get_window_title(hwnd: HWND) -> String {
    let len = unsafe { GetWindowTextLengthW(hwnd) };
    if len == 0 {
        return String::new();
    }
    let mut buf = vec![0u16; (len + 1) as usize];
    let active_len = unsafe { GetWindowTextW(hwnd, buf.as_mut_ptr(), buf.len() as i32) };
    if active_len > 0 {
        let os_str = OsString::from_wide(&buf[..active_len as usize]);
        os_str.to_string_lossy().into_owned()
    } else {
        String::new()
    }
}

fn get_exe_path(hwnd: HWND) -> String {
    let mut pid: DWORD = 0;
    unsafe { GetWindowThreadProcessId(hwnd, &mut pid) };
    if pid == 0 {
        return String::new();
    }

    // PROCESS_QUERY_LIMITED_INFORMATION = 0x1000
    let process_handle = unsafe { OpenProcess(0x1000, 0, pid) };
    if process_handle.is_null() {
        return String::new();
    }

    let mut len = 1024;
    let mut buf = vec![0u16; len as usize];
    let success = unsafe {
        QueryFullProcessImageNameW(process_handle, 0, buf.as_mut_ptr(), &mut len)
    };
    unsafe { CloseHandle(process_handle) };

    if success != 0 {
        let os_str = OsString::from_wide(&buf[..len as usize]);
        os_str.to_string_lossy().into_owned()
    } else {
        String::new()
    }
}

unsafe extern "system" fn enum_windows_with_handles_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
    if IsWindowVisible(hwnd) == 0 {
        return 1;
    }

    let title = get_window_title(hwnd);
    if title.is_empty() {
        return 1;
    }

    let root = GetAncestor(hwnd, GA_ROOTOWNER);
    if root != hwnd {
        return 1;
    }

    if title == "Program Manager" || title == "Start" || title == "Settings" {
        return 1;
    }

    let exe_path = get_exe_path(hwnd);
    if exe_path.is_empty() {
        return 1;
    }

    let mut rect = RECT {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    };
    if GetWindowRect(hwnd, &mut rect) != 0 {
        let x = rect.left;
        let y = rect.top;
        let w = rect.right - rect.left;
        let h = rect.bottom - rect.top;

        if w <= 0 || h <= 0 {
            return 1;
        }

        let app_name = Path::new(&exe_path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Unknown")
            .to_string();

        let list = &mut *(lparam as *mut Vec<WindowWithHandle>);
        list.push(WindowWithHandle {
            info: WindowInfo {
                app_name,
                title,
                exe_path,
                launch_args: None,
                x,
                y,
                w,
                h,
            },
            hwnd,
        });
    }

    1
}

pub fn enumerate_windows_with_handles() -> Vec<WindowWithHandle> {
    let mut list: Vec<WindowWithHandle> = Vec::new();
    let lparam = &mut list as *mut Vec<WindowWithHandle> as LPARAM;
    unsafe {
        EnumWindows(Some(enum_windows_with_handles_callback), lparam);
    }

    // Post-process to extract folder paths from File Explorer
    let explorer_paths = crate::explorer::get_explorer_paths();
    for item in &mut list {
        let app_name_lower = item.info.app_name.to_lowercase();
        if app_name_lower == "explorer.exe" {
            let hwnd_val = item.hwnd as isize;
            if let Some(path) = explorer_paths.get(&hwnd_val) {
                item.info.launch_args = Some(path.clone());
            }
        } else if app_name_lower == "msedge.exe"
            || app_name_lower == "chrome.exe"
            || app_name_lower == "firefox.exe"
            || app_name_lower == "brave.exe"
            || app_name_lower == "opera.exe"
        {
            item.info.launch_args = Some("--restore-last-session".to_string());
        } else if let Some(path) = crate::title_parser::extract_launch_args(&item.info.app_name, &item.info.title) {
            item.info.launch_args = Some(path);
        }
    }

    list
}

pub fn close_window(hwnd: HWND) {
    unsafe {
        winapi::um::winuser::PostMessageW(hwnd, winapi::um::winuser::WM_CLOSE, 0, 0);
    }
}

pub fn restore_windows(windows: Vec<WindowInfo>) {
    use std::collections::HashSet;
    use std::ffi::OsStr;
    use winapi::um::shellapi::ShellExecuteW;
    use winapi::um::winuser::{SWP_NOACTIVATE, SWP_NOZORDER, SWP_SHOWWINDOW, SW_SHOWNORMAL};

    let mut launched_browsers = HashSet::new();

    for w in &windows {
        if Path::new(&w.exe_path).exists() {
            let app_name_lower = w.app_name.to_lowercase();
            let is_browser = app_name_lower == "msedge.exe"
                || app_name_lower == "chrome.exe"
                || app_name_lower == "firefox.exe"
                || app_name_lower == "brave.exe"
                || app_name_lower == "opera.exe";

            if is_browser {
                if launched_browsers.contains(&w.exe_path) {
                    continue; // Skip duplicate launches for browser session
                }
                launched_browsers.insert(w.exe_path.clone());
            }

            let path_wide: Vec<u16> = OsStr::new(&w.exe_path)
                .encode_wide()
                .chain(Some(0))
                .collect();

            let params_wide: Option<Vec<u16>> = if let Some(ref args) = w.launch_args {
                let final_args = if args.starts_with("--") {
                    args.clone()
                } else {
                    format!("\"{}\"", args)
                };
                Some(OsStr::new(&final_args)
                    .encode_wide()
                    .chain(Some(0))
                    .collect())
            } else {
                None
            };

            let params_ptr = params_wide
                .as_ref()
                .map(|p| p.as_ptr())
                .unwrap_or(std::ptr::null());

            let res = unsafe {
                ShellExecuteW(
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    path_wide.as_ptr(),
                    params_ptr,
                    std::ptr::null_mut(),
                    SW_SHOWNORMAL,
                )
            };
            if (res as isize) <= 32 && !params_ptr.is_null() {
                unsafe {
                    ShellExecuteW(
                        std::ptr::null_mut(),
                        std::ptr::null_mut(),
                        path_wide.as_ptr(),
                        std::ptr::null_mut(),
                        std::ptr::null_mut(),
                        SW_SHOWNORMAL,
                    );
                }
            }
        }
    }

    // Delay repositioning on a background thread to let windows launch
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(1500));
        let mut active_windows = enumerate_windows_with_handles();
        for saved_w in &windows {
            if let Some(pos) = active_windows
                .iter()
                .position(|act| act.info.exe_path == saved_w.exe_path)
            {
                let active = active_windows.remove(pos);
                unsafe {
                    winapi::um::winuser::SetWindowPos(
                        active.hwnd,
                        std::ptr::null_mut(),
                        saved_w.x,
                        saved_w.y,
                        saved_w.w,
                        saved_w.h,
                        SWP_NOZORDER | SWP_NOACTIVATE | SWP_SHOWWINDOW,
                    );
                }
            }
        }
    });
}
