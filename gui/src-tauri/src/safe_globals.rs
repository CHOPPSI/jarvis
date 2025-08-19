// Safe accessors for global static variables in GUI
// This module provides safe wrappers around OnceCell globals to avoid unwrap() panics

use std::path::PathBuf;
use platform_dirs::AppDirs;
use crate::{APP_DIRS, APP_CONFIG_DIR, APP_LOG_DIR, DB};
use crate::db::structs::Settings;

/// Safe accessor for APP_DIRS
pub fn get_app_dirs() -> Result<&'static AppDirs, &'static str> {
    APP_DIRS.get().ok_or("Application directories not initialized")
}

/// Safe accessor for APP_CONFIG_DIR
pub fn get_config_dir() -> Result<&'static PathBuf, &'static str> {
    APP_CONFIG_DIR.get().ok_or("Config directory not initialized")
}

/// Safe accessor for APP_LOG_DIR  
pub fn get_log_dir() -> Result<&'static PathBuf, &'static str> {
    APP_LOG_DIR.get().ok_or("Log directory not initialized")
}

/// Safe accessor for DB
pub fn get_db() -> Result<&'static Settings, &'static str> {
    DB.get().ok_or("Database not initialized")
}

/// Safe accessor for config directory path as string
pub fn get_config_dir_display() -> Result<String, &'static str> {
    Ok(get_config_dir()?.display().to_string())
}

/// Safe accessor for log directory path as string  
pub fn get_log_dir_display() -> Result<String, &'static str> {
    Ok(get_log_dir()?.display().to_string())
}