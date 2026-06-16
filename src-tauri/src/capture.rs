use std::path::Path;
use xcap::Monitor;

pub fn capture_screenshot(output_path: &Path) -> Result<(), String> {
    let monitors = Monitor::all().map_err(|e| format!("Failed to get monitors: {}", e))?;
    if let Some(monitor) = monitors.first() {
        let image = monitor
            .capture_image()
            .map_err(|e| format!("Failed to capture image: {}", e))?;
        image
            .save(output_path)
            .map_err(|e| format!("Failed to save screenshot: {}", e))?;
        Ok(())
    } else {
        Err("No monitors found to capture".to_string())
    }
}
