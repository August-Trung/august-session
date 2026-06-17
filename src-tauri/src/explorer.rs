use std::collections::HashMap;
use windows::core::ComInterface;
use windows::Win32::System::Com::{
    CoCreateInstance, CoInitializeEx, CLSCTX_LOCAL_SERVER, COINIT_APARTMENTTHREADED,
};
use windows::Win32::System::Com::VARIANT;
use windows::Win32::UI::Shell::{IShellWindows, ShellWindows, IWebBrowser2};

fn create_i32_variant(n: i32) -> VARIANT {
    use std::mem::ManuallyDrop;
    use windows::Win32::System::Com::{VARIANT_0_0, VT_I4};
    
    let mut variant = VARIANT::default();
    let mut v00 = VARIANT_0_0::default();
    v00.vt = VT_I4;
    v00.Anonymous.lVal = n;
    variant.Anonymous.Anonymous = ManuallyDrop::new(v00);
    variant
}

pub fn get_explorer_paths() -> HashMap<isize, String> {
    let mut paths = HashMap::new();
    unsafe {
        // Initialize COM for the calling thread
        let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);

        if let Ok(shell_windows) = CoCreateInstance::<_, IShellWindows>(&ShellWindows, None, CLSCTX_LOCAL_SERVER) {
            if let Ok(count) = shell_windows.Count() {
                for i in 0..count {
                    let index = create_i32_variant(i);
                    if let Ok(dispatch) = shell_windows.Item(index) {
                        if let Ok(browser) = dispatch.cast::<IWebBrowser2>() {
                            let hwnd = match browser.HWND() {
                                Ok(h) => h.0,
                                Err(_) => continue,
                            };
                            if let Ok(url_bstr) = browser.LocationURL() {
                                let url = url_bstr.to_string();
                                if url.starts_with("file://") {
                                    // Normalizing the URL path
                                    // file:///C:/Users/... -> C:\Users\...
                                    // file://///server/share/... -> \\server\share\... (UNC path)
                                    let mut path = url.as_str();
                                    if path.starts_with("file:///") {
                                        path = &path[8..];
                                    } else if path.starts_with("file://") {
                                        path = &path[7..];
                                    }
                                    
                                    let mut decoded_path = url_decode(path).replace('/', "\\");
                                    // If it is a UNC path (e.g. Server\Share), restore prefix
                                    if url.starts_with("file://") && !url.starts_with("file:///") {
                                        decoded_path = format!("\\\\{}", decoded_path);
                                    }
                                    
                                    paths.insert(hwnd, decoded_path);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    paths
}

fn url_decode(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '%' {
            let mut hex = String::new();
            if let Some(&h1) = chars.peek() {
                hex.push(h1);
                let _ = chars.next();
            }
            if let Some(&h2) = chars.peek() {
                hex.push(h2);
                let _ = chars.next();
            }
            if hex.len() == 2 {
                if let Ok(val) = u8::from_str_radix(&hex, 16) {
                    result.push(val as char);
                    continue;
                }
            }
            result.push('%');
            result.push_str(&hex);
        } else {
            result.push(c);
        }
    }
    result
}
