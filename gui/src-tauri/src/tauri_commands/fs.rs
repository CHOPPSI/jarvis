use std::process::Command;
use std::fs::metadata;
use std::path::PathBuf;

// taken from https://github.com/tauri-apps/tauri/issues/4062#issuecomment-1338048169
#[tauri::command]
pub fn show_in_folder(path: String) -> Result<(), String> {
  #[cfg(target_os = "windows")]
  {
    Command::new("explorer")
        .args(["/select,", &path]) // The comma after select is not a typo
        .spawn()
        .map_err(|e| format!("Failed to open explorer: {}", e))?;
  }

  #[cfg(target_os = "linux")]
  {
    if path.contains(",") {
      // see https://gitlab.freedesktop.org/dbus/dbus/-/issues/76
      let new_path = match metadata(&path) {
        Ok(meta) => {
          if meta.is_dir() {
            path
          } else {
            let mut path2 = PathBuf::from(path);
            path2.pop();
            path2.into_os_string()
              .into_string()
              .map_err(|_| "Failed to convert path to string")?
          }
        }
        Err(e) => return Err(format!("Failed to get file metadata: {}", e)),
      };
      Command::new("xdg-open")
          .arg(&new_path)
          .spawn()
          .map_err(|e| format!("Failed to open xdg-open: {}", e))?;
    } else {
      Command::new("dbus-send")
          .args(["--session", "--dest=org.freedesktop.FileManager1", "--type=method_call",
                "/org/freedesktop/FileManager1", "org.freedesktop.FileManager1.ShowItems",
                format!("array:string:\"file://{path}\"").as_str(), "string:\"\""])
          .spawn()
          .map_err(|e| format!("Failed to open dbus-send: {}", e))?;
    }
  }

  #[cfg(target_os = "macos")]
  {
    Command::new("open")
        .args(["-R", &path])
        .spawn()
        .map_err(|e| format!("Failed to open finder: {}", e))?;
  }
  
  Ok(())
}