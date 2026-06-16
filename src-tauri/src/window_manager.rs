use serde::{Deserialize, Serialize};
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
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
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
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

unsafe extern "system" fn enum_windows_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
    if IsWindowVisible(hwnd) == 0 {
        return 1; // Continue
    }

    let title = get_window_title(hwnd);
    if title.is_empty() {
        return 1;
    }

    // Check if it's the root owner window
    let root = GetAncestor(hwnd, GA_ROOTOWNER);
    if root != hwnd {
        return 1;
    }

    // Filter out common Windows shell overlays
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

        let list = &mut *(lparam as *mut Vec<WindowInfo>);
        list.push(WindowInfo {
            app_name,
            title,
            exe_path,
            x,
            y,
            w,
            h,
        });
    }

    1 // Continue
}

pub fn enumerate_windows() -> Vec<WindowInfo> {
    let mut list = Vec::new();
    let lparam = &mut list as *mut Vec<WindowInfo> as LPARAM;
    unsafe {
        EnumWindows(Some(enum_windows_callback), lparam);
    }
    list
}
