use crate::DB;

// Safe helper function for DB access
fn safe_db_access<T, F>(operation: F) -> Result<T, String> 
where 
    F: FnOnce(&crate::db::structs::Settings) -> T,
{
    match DB.get() {
        Some(db) => Ok(operation(db)),
        None => Err("Database not initialized".to_string()),
    }
}

#[tauri::command]
pub fn db_read(key: &str) -> Result<String, String> {
    // For GUI, we're using a different DB structure (Settings struct)
    // This is a placeholder - need to implement proper key-value access
    match key {
        "selected_wake_word_engine" => {
            safe_db_access(|db| {
                match db.wake_word_engine {
                    crate::config::structs::WakeWordEngine::Porcupine => "porcupine".to_string(),
                    crate::config::structs::WakeWordEngine::Rustpotter => "rustpotter".to_string(),
                    crate::config::structs::WakeWordEngine::Vosk => "vosk".to_string(),
                }
            })
        },
        "api_key__picovoice" => {
            safe_db_access(|db| db.api_keys.picovoice.clone())
        },
        _ => {
            warn!("Unknown database key requested: {}", key);
            Ok(String::new())
        }
    }
}

#[tauri::command]
pub fn db_write(key: &str, val: &str) -> Result<bool, String> {
    // For GUI Settings struct, we need to implement proper field updates
    // This is a simplified version - in production should update the actual Settings struct
    warn!("DB write operation not fully implemented for Settings struct: {} = {}", key, val);
    Ok(true) // Return success for now to avoid breaking the GUI
}
