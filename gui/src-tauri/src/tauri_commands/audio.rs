use pv_recorder::RecorderBuilder;

#[tauri::command]
pub fn pv_get_audio_devices() -> Result<Vec<String>, String> {
    let audio_devices = RecorderBuilder::default().get_audio_devices();
    match audio_devices {
        Ok(audio_devices) => Ok(audio_devices),
        Err(err) => Err(format!("Failed to get audio devices: {}", err)),
    }
}

#[tauri::command]
pub fn pv_get_audio_device_name(idx: i32) -> Result<String, String> {
    let audio_devices = RecorderBuilder::default().get_audio_devices();
    match audio_devices {
        Ok(audio_devices) => {
            if audio_devices.is_empty() {
                return Err("No audio devices found".to_string());
            }
            
            // Return device at specific index if it exists
            if let Some(device) = audio_devices.get(idx as usize) {
                return Ok(device.to_string());
            }
            
            // Return first device as fallback
            Ok(audio_devices[0].to_string())
        }
        Err(err) => Err(format!("Failed to get audio devices: {}", err)),
    }
}
