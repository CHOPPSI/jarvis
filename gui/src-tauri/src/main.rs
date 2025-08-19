// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;

use once_cell::sync::OnceCell;
use platform_dirs::{AppDirs};

// expose the config
mod config;

// include log
#[macro_use]
extern crate simple_log;
mod log;

// include db
mod db;

// include tray
mod tray;

// include tauri commands
// mod tauri_commands;

// include safe globals
mod safe_globals;

// some global data
static APP_DIRS: OnceCell<AppDirs> = OnceCell::new();
static APP_CONFIG_DIR: OnceCell<PathBuf> = OnceCell::new();
static APP_LOG_DIR: OnceCell<PathBuf> = OnceCell::new();
static DB: OnceCell<db::structs::Settings> = OnceCell::new();

fn main() -> Result<(), String> {
    // initialize directories
    config::init_dirs()?;

    // initialize logging
    log::init_logging()?;

    // log some base info
    info!("Starting Jarvis v{} ...", config::APP_VERSION.unwrap_or("unknown"));
    
    match safe_globals::get_config_dir_display() {
        Ok(dir) => info!("Config directory is: {}", dir),
        Err(e) => error!("Failed to get config directory: {}", e),
    }
    
    match safe_globals::get_log_dir_display() {
        Ok(dir) => info!("Log directory is: {}", dir),
        Err(e) => error!("Failed to get log directory: {}", e),
    }

    // initialize database (settings)
    DB.set(db::init_settings());

    Ok(())
}